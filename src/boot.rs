use bon::Builder;

use crate::common::OnOff;
use crate::to_command::ToArg;
use crate::to_command::ToCommand;

/// Specify boot order drives as a string of drive letters. Valid drive
/// letters depend on the target architecture. The x86 PC uses: a, b
/// (floppy 1 and 2), c (first hard disk), d (first CD-ROM), n-p
/// (Etherboot from network adapter 1-4), hard disk boot is the default.
/// To apply a particular boot order only on the first startup, specify
/// it via ``once``. Note that the ``order`` or ``once`` parameter
/// should not be used together with the ``bootindex`` property of
/// devices, since the firmware implementations normally do not support
/// both at the same time.
///
/// Interactive boot menus/prompts can be enabled via ``menu=on`` as far
/// as firmware/BIOS supports them. The default is non-interactive boot.
///
/// A splash picture could be passed to bios, enabling user to show it
/// as logo, when option splash=sp\_name is given and menu=on, If
/// firmware/BIOS supports them. Currently Seabios for X86 system
/// support it. limitation: The splash file could be a jpeg file or a
/// BMP file in 24 BPP format(true color). The resolution should be
/// supported by the SVGA mode, so the recommended is 320x240, 640x480,
/// 800x640.
///
/// A timeout could be passed to bios, guest will pause for rb\_timeout
/// ms when boot failed, then reboot. If rb\_timeout is '-1', guest will
/// not reboot, qemu passes '-1' to bios by default. Currently Seabios
/// for X86 system support it.
///
/// Do strict boot via ``strict=on`` as far as firmware/BIOS supports
/// it. This only effects when boot priority is changed by bootindex
/// options. The default is non-strict boot.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder)]
pub struct Boot {
    order: Option<String>,
    once: Option<String>,
    menu: Option<OnOff>,
    splash: Option<String>,
    splash_time: Option<usize>,
    reboot_timeout: Option<usize>,
    strict: Option<OnOff>,
}

impl ToCommand for Boot {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        if self.order.is_some()
            || self.once.is_some()
            || self.menu.is_some()
            || self.splash.is_some()
            || self.splash_time.is_some()
            || self.reboot_timeout.is_some()
            || self.strict.is_some()
        {
            cmd.push("-boot".to_string());
        }

        let mut args = vec![];

        if let Some(order) = &self.order {
            args.push(format!("order={}", order));
        }
        if let Some(once) = &self.once {
            args.push(format!("once={}", once));
        }
        if let Some(menu) = &self.menu {
            args.push(format!("menu={}", menu.to_arg()));
        }
        if let Some(splash) = &self.splash {
            args.push(format!("splash={}", splash));
        }
        if let Some(splash_time) = &self.splash_time {
            args.push(format!("splash-time={}", splash_time));
        }
        if let Some(reboot_timeout) = &self.reboot_timeout {
            args.push(format!("reboot-timeout={}", reboot_timeout));
        }
        if let Some(strict) = &self.strict {
            args.push(format!("strict={}", strict.to_arg()));
        }
        cmd.push(args.join(","));
        cmd
    }
}
