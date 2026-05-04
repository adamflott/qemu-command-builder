use std::fmt::{Display, Formatter};
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::common::OnOff;
use crate::parsers::DELIM_COMMA;
use crate::qao;
use crate::shell_string::{ShellString, ShellStringError};
use crate::to_command::{ToArg, ToCommand};

pub(crate) const ARG_MON: &str = "-mon";

const KEY_CHARDEV: &str = "chardev=";
const KEY_MODE: &str = "mode=";
const KEY_PRETTY: &str = "pretty=";

/// Supported QEMU monitor protocol modes.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum ReadlineControl {
    /// Human Monitor Protocol.
    Readline,
    /// QEMU Monitor Protocol.
    Control,
}

impl Display for ReadlineControl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadlineControl::Readline => write!(f, "readline"),
            ReadlineControl::Control => write!(f, "control"),
        }
    }
}

impl FromStr for ReadlineControl {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "readline" => Ok(ReadlineControl::Readline),
            "control" => Ok(ReadlineControl::Control),
            _ => Err(()),
        }
    }
}
impl ToArg for ReadlineControl {
    fn to_arg(&self) -> &str {
        match self {
            ReadlineControl::Readline => "readline",
            ReadlineControl::Control => "control",
        }
    }
}
/// A `-mon` monitor endpoint definition.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Mon {
    /// The backing chardev id or name.
    #[builder(into)]
    chardev: ShellString,
    /// The monitor protocol mode.
    mode: Option<ReadlineControl>,
    /// QMP pretty-printing toggle.
    pretty: Option<OnOff>,
}

impl ToCommand for Mon {
    fn command(&self) -> String {
        ARG_MON.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];
        args.push(format!("{}{}", KEY_CHARDEV, self.chardev.as_ref()));
        qao!(&self.mode, args, KEY_MODE);
        qao!(&self.pretty, args, KEY_PRETTY);
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Mon {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_mon(s).map_err(ShellStringError::new)
    }
}

fn parse_mon(s: &str) -> Result<Mon, String> {
    let mut parts = s.split(DELIM_COMMA);
    let first = parts.next().ok_or_else(|| "empty monitor argument".to_string())?;

    let chardev = if let Some(value) = first.strip_prefix(KEY_CHARDEV) {
        ShellString::new(value)
    } else if first.contains('=') {
        return Err(format!("unsupported monitor option in first position: {first}"));
    } else {
        ShellString::new(first)
    };

    let mut mode = None;
    let mut pretty = None;

    for part in parts {
        if part == "pretty" {
            pretty = Some(OnOff::On);
            continue;
        }

        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid monitor option: {part}"))?;
        match key {
            "chardev" => return Err("chardev= is only valid as the first -mon component".to_string()),
            "mode" => {
                mode = Some(value.parse::<ReadlineControl>().map_err(|_| format!("invalid mode value: {value}"))?);
            }
            "pretty" => {
                pretty = Some(value.parse::<OnOff>().map_err(|_| format!("invalid pretty value: {value}"))?);
            }
            other => return Err(format!("unsupported monitor option: {other}")),
        }
    }

    Ok(Mon { chardev, mode, pretty })
}
