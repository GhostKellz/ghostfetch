use std::process::{Command, Output, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Default timeout for external helper commands.
///
/// External tools (notably `kscreen-doctor`) can block indefinitely when the
/// service they query is wedged. Capping every spawn ensures one stuck helper
/// can never freeze the whole program.
const CMD_TIMEOUT: Duration = Duration::from_secs(2);

/// Run an external command, returning its output, or `None` if it cannot be
/// spawned or does not finish within [`CMD_TIMEOUT`].
pub fn run_cmd(program: &str, args: &[&str]) -> Option<Output> {
    run_cmd_timeout(program, args, CMD_TIMEOUT)
}

/// Run an external command with an explicit timeout.
///
/// Output is drained on a worker thread so a child that fills the pipe buffer
/// cannot deadlock the caller. If the timeout elapses first, the child is
/// killed and the worker reaps it.
pub fn run_cmd_timeout(program: &str, args: &[&str], timeout: Duration) -> Option<Output> {
    let child = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;

    let pid = child.id();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(child.wait_with_output());
    });

    match rx.recv_timeout(timeout) {
        Ok(result) => result.ok(),
        Err(_) => {
            // Timed out: kill the child so the worker thread can reap it.
            unsafe {
                libc::kill(pid as libc::pid_t, libc::SIGKILL);
            }
            None
        }
    }
}

/// Strip ANSI escape codes from a string and return clean text.
pub fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip escape sequence
            if chars.peek() == Some(&'[') {
                chars.next();
                // Skip until we hit a letter
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Extract version number from a version string line.
/// Looks for parts starting with a digit or 'v'.
#[allow(dead_code)]
pub fn parse_version_from_line(line: &str) -> Option<&str> {
    line.split_whitespace().find(|part| {
        part.chars()
            .next()
            .map(|c| c.is_ascii_digit() || c == 'v')
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi_codes() {
        assert_eq!(strip_ansi_codes("hello"), "hello");
        assert_eq!(strip_ansi_codes("\x1b[31mred\x1b[0m"), "red");
        assert_eq!(
            strip_ansi_codes("\x1b[1;32mbold green\x1b[0m"),
            "bold green"
        );
    }

    #[test]
    fn test_parse_version_from_line() {
        assert_eq!(parse_version_from_line("zsh 5.9"), Some("5.9"));
        assert_eq!(parse_version_from_line("v1.2.3"), Some("v1.2.3"));
        assert_eq!(parse_version_from_line("hello world"), None);
        assert_eq!(parse_version_from_line("program 1.0.0"), Some("1.0.0"));
    }

    #[test]
    fn test_run_cmd_captures_output() {
        let output = run_cmd("echo", &["hello"]).expect("echo should run");
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "hello");
    }

    #[test]
    fn test_run_cmd_missing_binary() {
        assert!(run_cmd("ghostfetch-does-not-exist-xyz", &[]).is_none());
    }

    #[test]
    fn test_run_cmd_timeout_kills_hung_child() {
        // `sleep 5` far exceeds the 100ms cap, so we must time out (not block).
        let start = std::time::Instant::now();
        let result = run_cmd_timeout("sleep", &["5"], Duration::from_millis(100));
        assert!(result.is_none());
        assert!(
            start.elapsed() < Duration::from_secs(2),
            "timeout should return promptly, took {:?}",
            start.elapsed()
        );
    }
}
