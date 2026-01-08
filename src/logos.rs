use colored::{ColoredString, Colorize};

pub struct DistroLogo {
    pub art: &'static str,
    pub width: usize,
    pub primary_color: fn(&str) -> ColoredString,
    pub secondary_color: fn(&str) -> ColoredString,
}

fn cyan(s: &str) -> ColoredString { s.cyan() }
fn blue(s: &str) -> ColoredString { s.blue() }
fn white(s: &str) -> ColoredString { s.white() }
fn red(s: &str) -> ColoredString { s.red() }
fn green(s: &str) -> ColoredString { s.green() }
fn yellow(s: &str) -> ColoredString { s.yellow() }
fn magenta(s: &str) -> ColoredString { s.magenta() }
fn bright_blue(s: &str) -> ColoredString { s.bright_blue() }

pub fn get_logo(distro_id: &str) -> DistroLogo {
    let id = distro_id.to_lowercase();

    // Arch-based distros (check specific ones first)
    if id.contains("cachyos") || id.contains("cachy") {
        cachyos_logo()
    } else if id.contains("endeavour") {
        endeavouros_logo()
    } else if id.contains("artix") {
        arch_logo() // Uses Arch logo with same colors
    } else if id.contains("arch") {
        arch_logo()
    // Fedora-based distros
    } else if id.contains("bazzite") {
        bazzite_logo()
    } else if id.contains("nobara") {
        nobara_logo()
    } else if id.contains("fedora") {
        fedora_logo()
    // Other distros
    } else if id.contains("ubuntu") {
        ubuntu_logo()
    } else if id.contains("debian") {
        debian_logo()
    } else if id.contains("pop") {
        popos_logo()
    } else if id.contains("manjaro") {
        manjaro_logo()
    } else if id.contains("mint") {
        mint_logo()
    } else if id.contains("opensuse") || id.contains("suse") {
        opensuse_logo()
    } else if id.contains("gentoo") {
        gentoo_logo()
    } else if id.contains("nixos") {
        nixos_logo()
    } else if id.contains("void") {
        void_logo()
    } else if id.contains("alpine") {
        alpine_logo()
    } else if id.contains("proxmox") {
        proxmox_logo()
    } else {
        linux_logo()
    }
}

fn arch_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
                   -`
                  .o+`
                 `ooo/
                `+oooo:
               `+oooooo:
               -+oooooo+:
             `/:-:++oooo+:
            `/++++/+++++++:
           `/++++++++++++++:
          `/+++ooooooooooooo/`
         ./ooosssso++osssssso+`
        .oossssso-````/ossssss+`
       -osssssso.      :ssssssso.
      :osssssss/        osssso+++.
     /ossssssss/        +ssssooo/-
   `/ossssso+/:-        -:/+osssso+-
  `+sso+:-`                 `.-/+oso:
 `++:.                           `-/+/
 .`                                 `/
"#,
        width: 40,
        primary_color: cyan,
        secondary_color: blue,
    }
}

fn ubuntu_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
            .-/+oossssoo+/-.
        `:+ssssssssssssssssss+:`
      -+ssssssssssssssssssyyssss+-
    .ossssssssssssssssssdMMMNysssso.
   /ssssssssssshdmmNNmmyNMMMMhssssss/
  +ssssssssshmydMMMMMMMNddddyssssssss+
 /sssssssshNMMMyhhyyyyhmNMMMNhssssssss/
.ssssssssdMMMNhsssssssssshNMMMdssssssss.
+sssshhhyNMMNyssssssssssssyNMMMysssssss+
ossyNMMMNyMMhsssssssssssssshmmmhssssssso
ossyNMMMNyMMhsssssssssssssshmmmhssssssso
+sssshhhyNMMNyssssssssssssyNMMMysssssss+
.ssssssssdMMMNhsssssssssshNMMMdssssssss.
 /sssssssshNMMMyhhyyyyhdNMMMNhssssssss/
  +sssssssssdmydMMMMMMMMddddyssssssss+
   /ssssssssssshdmNNNNmyNMMMMhssssss/
    .ossssssssssssssssssdMMMNysssso.
      -+sssssssssssssssssyyyssss+-
        `:+ssssssssssssssssss+:`
            .-/+oossssoo+/-.
"#,
        width: 42,
        primary_color: red,
        secondary_color: white,
    }
}

fn debian_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
       _,met$$$$$gg.
    ,g$$$$$$$$$$$$$$$P.
  ,g$$P"        """Y$$.".
 ,$$P'              `$$$.
',$$P       ,ggs.     `$$b:
`d$$'     ,$P"'   .    $$$
 $$P      d$'     ,    $$P
 $$:      $$.   -    ,d$$'
 $$;      Y$b._   _,d$P'
 Y$$.    `.`"Y$$$$P"'
 `$$b      "-.__
  `Y$$
   `Y$$.
     `$$b.
       `Y$$b.
          `"Y$b._
              `"""
"#,
        width: 35,
        primary_color: red,
        secondary_color: white,
    }
}

fn fedora_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
             .',;::::;,'.
         .';:cccccccccccc:;,.
      .;cccccccccccccccccccccc;.
    .:cccccccccccccccccccccccccc:.
  .;ccccccccccccc;.:dddl:.;ccccccc;.
 .:ccccccccccccc;OWMKOOXMWd;ccccccc:.
.:ccccccccccccc;KMMc;cc;xMMc;ccccccc:.
,cccccccccccccc;MMM.;cc;;WW:;cccccccc,
:cccccccccccccc;MMM.;cccccccccccccccc:
:ccccccc;oxOOOo;MMM0OOk.;cccccccccccc:
cccccc;0MMKxdd:;MMMkddc.;cccccccccccc;
ccccc;XM0';cccc;MMM.;cccccccccccccccc'
ccccc;MMo;ccccc;MMW.;ccccccccccccccc;
ccccc;0MNc.ccc.xMMd;ccccccccccccccc;
cccccc;dNMWXXXWM0:;cccccccccccccc:,
cccccccc;.:odl:.;cccccccccccccc:,.
:cccccccccccccccccccccccccccc:'.
.:cccccccccccccccccccc:;,..
  '::cccccccccccccc::;,.
"#,
        width: 42,
        primary_color: bright_blue,
        secondary_color: white,
    }
}

fn popos_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
             /////////////
         /////////////////////
      ///////*767////////////////
    //////7676767676*//////////////
   /////76767//7676767//////////////
  /////767676////*76767///////////////
 ///////767676///76767.///7676*///////
/////////767676//76767///767676////////
//////////76767676767////76767/////////
///////////76767676//////7676//////////
////////////,7676,///////767///////////
//////////////*7676///////76////////////
///////////////7676////////////////////
 ///////////////7676///767////////////
  //////////////////////'////////////
   //////.7676767676767676767,//////
    /////767676767676767676767/////
      ///////////////////////////
         /////////////////////
             /////////////
"#,
        width: 42,
        primary_color: cyan,
        secondary_color: white,
    }
}

fn manjaro_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
 $$$$$$$$$$$$$$$$  $$$$$$$$
 $$$$$$$$$$$$$$$$  $$$$$$$$
 $$$$$$$$$$$$$$$$  $$$$$$$$
 $$$$$$$$$$$$$$$$  $$$$$$$$
 $$$$$$$$          $$$$$$$$
 $$$$$$$$  $$$$$$  $$$$$$$$
 $$$$$$$$  $$$$$$  $$$$$$$$
 $$$$$$$$  $$$$$$  $$$$$$$$
 $$$$$$$$  $$$$$$  $$$$$$$$
 $$$$$$$$  $$$$$$  $$$$$$$$
 $$$$$$$$  $$$$$$  $$$$$$$$
 $$$$$$$$  $$$$$$  $$$$$$$$
 $$$$$$$$  $$$$$$  $$$$$$$$
 $$$$$$$$  $$$$$$  $$$$$$$$
"#,
        width: 36,
        primary_color: green,
        secondary_color: green,
    }
}

fn mint_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
             ...-:::::-...
          .-MMMMMMMMMMMMMMM-.
      .-MMMM`..-:::::::-..`MMMM-.
    .:MMMM.:MMMMMMMMMMMMMMM:.MMMM:.
   -MMM-M---MMMMMMMMMMMMMMMMMMM.MMM-
 `:MMM:MM`  :MMMM:....::-...-MMMM:MMM:`
 :MMM:MMM`  :MM:`  ``    ``  `:MMM:MMM:
.MMM.MMMM`  :MM.  -MM.  .MM-  `MMMM.MMM.
:MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM-MMM:
:MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM:MMM:
:MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM-MMM:
.MMM.MMMM`  :MM:--:MM:--:MM:  `MMMM.MMM.
 :MMM:MMM-  `-MMMMMMMMMMMM-`  -MMM-MMM:
  :MMM:MMM:`                `:MMM:MMM:
   .MMM.MMMM:--------------:MMMM.MMM.
     '-MMMM.-MMMMMMMMMMMMMMM-.MMMM-'
       '.-MMMM``--:::::--``MMMM-.'
            '-MMMMMMMMMMMMM-'
               ``-:::::-``
"#,
        width: 43,
        primary_color: green,
        secondary_color: white,
    }
}

fn opensuse_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
           .;ldkO0000Okdl;.
       .;d00xl:^''''''^:ok00d;.
     .d00l'                'o00d.
   .d0Kd'  Okxol:;,.          :O0d.
  .OKKK0kOKKKKKKKKKKOxo:,      lKO.
 ,0KKKKKKKKKKKKKKKK0P^,,,^dx:    ;00,
.OKKKKKKKKKKKKKKKK: kKx..dd lKd   cKO.
:KKKKKKKKKKKKKKKKK; KMMc;cc;xMMc   'OK:
dKKKKKKKKKKKOx0KKKd ^0KKKO' kKKc   dKd
dKKKKKKKKKKKK;.;oOKx,..^..;kKKK0.  dKd
:KKKKKKKKKKKK0o;...^cdxxOK0O/^^'  .0K:
 kKKKKKKKKKKKKKKK0x;,,......,;od  lKk
 '0KKKKKKKKKKKKKKKKKKK00KKOo^  c00'
  'kKKOxddxkOO00000Okxoc;''   .dKk'
    l0Ko.                    .c00l'
     'l0Kk:.              .;xK0l'
        'lkK0xl:;,,,,;:ldO0kl'
            '^:ldxkkkkxdl:^'
"#,
        width: 40,
        primary_color: green,
        secondary_color: white,
    }
}

fn gentoo_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
         -/oyddmdhs+:.
     -odNMMMMMMMMNNmhy+-`
   -yNMMMMMMMMMMMNNNmmdhy+-
 `omMMMMMMMMMMMMNmdmmmmddhhy/`
 omMMMMMMMMMMMNhhyyo+hmdddhhhdo`
.ydMMMMMMMMMMdhs++so/smdddhhhhdm+`
 oyhhdNNMMMMMMMNdyooy+dmdddhhhyhNd.
  :oyhhdNNMMMMMMMNNNmmdddhhhhhymMh
    .:+sydNMMMMMNNNmmmdddhhhhhhmMmy
       /mMMMMMMNNNmmmdddhhhhhmMNhs:
    `oNMMMMMMMNNNmmmddddhhdmMNhs+`
  `sNMMMMMMMMNNNmmmdddddmNMmhs/.
 /NMMMMMMMMNNNNmmmdddmNMNdso:`
+MMMMMMMNNNNNmmmmdmNMNdso/-
yMMNNNNNNNmmmmmNNMmhs+/-`
/hMMNNNNNNNNMNdhs++/-`
`/ohdmmddhys+++/:.`
  `-//////:--.
"#,
        width: 38,
        primary_color: magenta,
        secondary_color: white,
    }
}

fn nixos_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
          ::::.    ':::::     ::::'
          ':::::    ':::::.  ::::'
            :::::     '::::.:::::
      .......:::::..... ::::::::
     ::::::::::::::::::. ::::::    ::::.
    ::::::::::::::::::::: :::::.  .::::'
           .....           ::::' :::::'
          :::::            '::' :::::'
 ........:googol'googol......googol:::::.googol
::::::::::::::::::::::::::::::::::. '::::googol.
 :::::::::::::::::::::::::::::::::::'  .googol:'
        .:::::::::::::::::::::::.googol ::::'
       .::::::::::::::::::::::::.googol::::'
      .::::::::::::::::::::::::.  ':::googol
     .::::::::::::::::::::::.googol '::::.
    .::::::::::::::::::.googol'  .::.
          .googol                googol
"#,
        width: 50,
        primary_color: blue,
        secondary_color: cyan,
    }
}

fn void_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
                __.;=====;.__
            _.=+==++=++=+=+===;.
             -=+++=+===+=+=+++++=_
        .     -=:``     `--==+=++==.
       _vi,    `            --+=++++:
      .uvnvi.       _._       -==+==+.
     .vvnvnI`    .;==|==;.     :|=||=|.
    +QmQQmpvvnv;_yYsyQQWUUQQQm #QmQ#:QQQWUV$QQm.
     -QQWQW+pvvowZ?.teleport   /QWQW.QQWW(: jQWQE
      -$teleportmU'  teleport  )mQQQ.mQQQC+;jWQQ@'
       -$WQ8teleport   teleport mWQQ.jQWQQgyyWW@!
         -1vvnvv.     `~+++`        ++|+++
          +vnvnnv,                 `-|===
           +vnvnvns.           .      :=-
            -Invnvvnsi..___..=sv=.     `
              +Invnvnvnnnnnnnnvvnn;.
                ~|Invnvnvvnvvvnnv}+`
                   -~|{*l}*|~
"#,
        width: 52,
        primary_color: green,
        secondary_color: white,
    }
}

fn alpine_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
       .hddddddddddddddddddddddh.
      :dddddddddddddddddddddddddd:
     /dddddddddddddddddddddddddddd/
    +dddddddddddddddddddddddddddddd+
  `sdddddddddddddddddddddddddddddddds`
 `ydddddddddddd++hdddddddddddddddddddy`
.hddddddddddd+`  `+ddddh:-sdddddddddddh.
hdddddddddd+`      `+y:    .sddddddddddh
ddddddddh+`   `//`   `.`     -sddddddddd
ddddddh+`   `/hddh/`   `:s-    -sddddddd
ddddh+`   `/+/dddddh/`   `+s-    -sddddd
ddd+`   `/o` :dddddddh/`   `oy-    .yddd
hdddyo+ohddyosdddddddddho+oydddy++ohdddh
.hddddddddddddddddddddddddddddddddddddh.
 `yddddddddddddddddddddddddddddddddddy`
  `sdddddddddddddddddddddddddddddddds`
    +dddddddddddddddddddddddddddddd+
     /dddddddddddddddddddddddddddd/
      :dddddddddddddddddddddddddd:
       .hddddddddddddddddddddddh.
"#,
        width: 42,
        primary_color: blue,
        secondary_color: white,
    }
}

fn proxmox_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
         .://:`              `://:.
       `hMMMMMMd/          /dMMMMMMh`
        `sMMMMMMMd:      :mMMMMMMMs`
`-/+oo+/:.yMMMMMMMh-  -hMMMMMMMy.:/+oo+/-`
`:oooooooo/-hMMMMMMMyyMMMMMMMh-/oooooooo:`
  `/oooooooo::mMMMMMMMMMMMMm::oooooooo/`
    ./ooooooo+- +NMMMMMMMMN+ -+ooooooo/.
      .+ooooooo+-`oNMMMMNo`-+ooooooo+.
        -+ooooooo/.`sMMs`./ooooooo+-
          :oooooooo/`.``/oooooooo:
          :oooooooo/`..`/oooooooo:
        -+ooooooo/.`sMMs`./ooooooo+-
      .+ooooooo+-`oNMMMMNo`-+ooooooo+.
    ./ooooooo+- +NMMMMMMMMN+ -+ooooooo/.
  `/oooooooo::mMMMMMMMMMMMMm:`:oooooooo/`
`:oooooooo/-hMMMMMMMyyMMMMMMMh-`/oooooooo:`
`-/+oo+/:.yMMMMMMMh-  -hMMMMMMMy.`:/+oo+/-`
        `sMMMMMMMm:      :dMMMMMMMs`
       `hMMMMMMd/          /dMMMMMMh`
         `://:`              `://:`
"#,
        width: 46,
        primary_color: white,
        secondary_color: yellow,
    }
}

fn linux_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
        #####
       #######
       ##O#O##
       #######
     ###########
    #############
   ###############
   ################
  #################
#####################
#####################
  #################
"#,
        width: 26,
        primary_color: white,
        secondary_color: yellow,
    }
}

fn cachyos_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
                   -`
                  .o+`
                 `ooo/
                `+oooo:
               `+oooooo:
               -+oooooo+:
             `/:-:++oooo+:
            `/++++/+++++++:
           `/++++++++++++++:
          `/+++ooooooooooooo/`
         ./ooosssso++osssssso+`
        .oossssso-````/ossssss+`
       -osssssso.      :ssssssso.
      :osssssss/        osssso+++.
     /ossssssss/        +ssssooo/-
   `/ossssso+/:-        -:/+osssso+-
  `+sso+:-`                 `.-/+oso:
 `++:.                           `-/+/
 .`                                 `/
"#,
        width: 40,
        primary_color: green,
        secondary_color: cyan,
    }
}

fn endeavouros_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
                     ./o.
                   ./sssso-
                 `:osssssss+-
               `:+sssssssssso/.
             `-/ossssssssssssso/.
           `-/+sssssssssssssssso+:`
         `-:/+sssssssssssssssssso+/.
       `.://osssssssssssssssssssso++-
      .://+ssssssssssssssssssssssso++:
    .:///ossssssssssssssssssssssssso++:
  `:////ssssssssssssssssssssssssssso+++.
`-////+ssssssssssssssssssssssssssso++++-
 `..-+oosssssssssssssssssssssssso+++++/`
   ./++++++++++++++++++++++++++++++/:.
  `:::::::::::::::::::::::::------``
"#,
        width: 46,
        primary_color: magenta,
        secondary_color: red,
    }
}

fn bazzite_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
             .',;::::;,'.
         .';:cccccccccccc:;,.
      .;cccccccccccccccccccccc;.
    .:cccccccccccccccccccccccccc:.
  .;ccccccccccccc;.:dddl:.;ccccccc;.
 .:ccccccccccccc;OWMKOOXMWd;ccccccc:.
.:ccccccccccccc;KMMc;cc;xMMc;ccccccc:.
,cccccccccccccc;MMM.;cc;;WW:;cccccccc,
:cccccccccccccc;MMM.;cccccccccccccccc:
:ccccccc;oxOOOo;MMM0OOk.;cccccccccccc:
cccccc;0MMKxdd:;MMMkddc.;cccccccccccc;
ccccc;XM0';cccc;MMM.;cccccccccccccccc'
ccccc;MMo;ccccc;MMW.;ccccccccccccccc;
ccccc;0MNc.ccc.xMMd;ccccccccccccccc;
cccccc;dNMWXXXWM0:;cccccccccccccc:,
cccccccc;.:odl:.;cccccccccccccc:,.
:cccccccccccccccccccccccccccc:'.
.:cccccccccccccccccccc:;,..
  '::cccccccccccccc::;,.
"#,
        width: 42,
        primary_color: magenta,
        secondary_color: white,
    }
}

fn nobara_logo() -> DistroLogo {
    DistroLogo {
        art: r#"
             .',;::::;,'.
         .';:cccccccccccc:;,.
      .;cccccccccccccccccccccc;.
    .:cccccccccccccccccccccccccc:.
  .;ccccccccccccc;.:dddl:.;ccccccc;.
 .:ccccccccccccc;OWMKOOXMWd;ccccccc:.
.:ccccccccccccc;KMMc;cc;xMMc;ccccccc:.
,cccccccccccccc;MMM.;cc;;WW:;cccccccc,
:cccccccccccccc;MMM.;cccccccccccccccc:
:ccccccc;oxOOOo;MMM0OOk.;cccccccccccc:
cccccc;0MMKxdd:;MMMkddc.;cccccccccccc;
ccccc;XM0';cccc;MMM.;cccccccccccccccc'
ccccc;MMo;ccccc;MMW.;ccccccccccccccc;
ccccc;0MNc.ccc.xMMd;ccccccccccccccc;
cccccc;dNMWXXXWM0:;cccccccccccccc:,
cccccccc;.:odl:.;cccccccccccccc:,.
:cccccccccccccccccccccccccccc:'.
.:cccccccccccccccccccc:;,..
  '::cccccccccccccc::;,.
"#,
        width: 42,
        primary_color: red,
        secondary_color: white,
    }
}
