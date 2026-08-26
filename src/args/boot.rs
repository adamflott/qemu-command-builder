use crate::parsers::ARG_BOOT;
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;
use winnow::Result;
use winnow::ascii::{alphanumeric1, dec_uint};
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::token::literal;

use crate::common::OnOff;
use crate::parsers::DELIM_COMMA;
use crate::shell_string::{ShellString, ShellStringError, shell_string_until_comma};
use crate::to_command::ToCommand;
use crate::{pco0, ppo0, qao};

const KEY_ORDER: &str = "order=";
const KEY_ONCE: &str = "once=";
const KEY_MENU: &str = "menu=";
const KEY_SPLASH: &str = "splash=";
const KEY_SPLASH_TIME: &str = "splash-time=";
const KEY_REBOOT_TIMEOUT: &str = "reboot-timeout=";
const KEY_STRICT: &str = "strict=";

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
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Boot {
    order: Option<ShellString>,
    once: Option<ShellString>,
    menu: Option<OnOff>,
    splash: Option<ShellString>,
    splash_time: Option<u64>,
    reboot_timeout: Option<u64>,
    strict: Option<OnOff>,
}

impl ToCommand for Boot {
    fn has_args(&self) -> bool {
        self.order.is_some() || self.once.is_some() || self.menu.is_some() || self.splash.is_some() || self.splash_time.is_some() || self.reboot_timeout.is_some() || self.strict.is_some()
    }

    fn command(&self) -> String {
        ARG_BOOT.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];

        if let Some(order) = &self.order {
            args.push(format!("{}{}", KEY_ORDER, order.as_ref()));
        }
        if let Some(once) = &self.once {
            args.push(format!("{}{}", KEY_ONCE, once.as_ref()));
        }
        qao!(&self.menu, args, KEY_MENU);
        if let Some(splash) = &self.splash {
            args.push(format!("{}{}", KEY_SPLASH, splash.as_ref()));
        }
        qao!(&self.splash_time, args, KEY_SPLASH_TIME);
        qao!(&self.reboot_timeout, args, KEY_REBOOT_TIMEOUT);
        qao!(&self.strict, args, KEY_STRICT);

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Boot {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = Self {
            order: None,
            once: None,
            menu: None,
            splash: None,
            splash_time: None,
            reboot_timeout: None,
            strict: None,
        };
        for part in s.split(DELIM_COMMA) {
            let (key, raw) = part.split_once('=').ok_or_else(|| ShellStringError::new(format!("invalid -boot option: {part}")))?;
            match key {
                "order" => value.order = Some(raw.parse::<ShellString>().map_err(ShellStringError::new)?),
                "once" => value.once = Some(raw.parse::<ShellString>().map_err(ShellStringError::new)?),
                "menu" => value.menu = Some(raw.parse::<OnOff>().map_err(|_| ShellStringError::new(format!("invalid menu value: {raw}")))?),
                "splash" => value.splash = Some(raw.parse::<ShellString>().map_err(ShellStringError::new)?),
                "splash-time" => value.splash_time = Some(raw.parse::<u64>().map_err(|e| ShellStringError::new(e.to_string()))?),
                "reboot-timeout" => value.reboot_timeout = Some(raw.parse::<u64>().map_err(|e| ShellStringError::new(e.to_string()))?),
                "strict" => value.strict = Some(raw.parse::<OnOff>().map_err(|_| ShellStringError::new(format!("invalid strict value: {raw}")))?),
                other => return Err(ShellStringError::new(format!("unsupported -boot option: {other}"))),
            }
        }
        Ok(value)
    }
}

pco0!(order, shell_string_until_comma, ShellString, KEY_ORDER);
pco0!(once, shell_string_until_comma, ShellString, KEY_ONCE);
pco0!(menu, alphanumeric1, OnOff, KEY_MENU);
pco0!(splash, shell_string_until_comma, ShellString, KEY_SPLASH);
ppo0!(splash_time, dec_uint, u64, KEY_SPLASH_TIME);
ppo0!(reboot_timeout, dec_uint, u64, KEY_REBOOT_TIMEOUT);
pco0!(strict, alphanumeric1, OnOff, KEY_STRICT);

pub fn boot(s: &mut &str) -> ModalResult<Boot> {
    let order = opt(order).parse_next(s)?;
    let once = opt(once).parse_next(s)?;
    let menu = opt(menu).parse_next(s)?;
    let splash = opt(splash).parse_next(s)?;
    let splash_time = opt(splash_time).parse_next(s)?;
    let reboot_timeout = opt(reboot_timeout).parse_next(s)?;
    let strict = opt(strict).parse_next(s)?;

    Ok(Boot {
        order,
        once,
        menu,
        splash,
        splash_time,
        reboot_timeout,
        strict,
    })
}
