use std::fmt::{Display, Formatter};
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;
use winnow::Result;
use winnow::ascii::alphanumeric1;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::token::literal;

use crate::common::OnOff;
use crate::parsers::DELIM_COMMA;
use crate::shell_string::{ShellString, ShellStringError, shell_string_until_comma};
use crate::to_command::{ToArg, ToCommand};
use crate::{pco, qao};

pub(crate) const ARG_MON: &str = "-mon";

const KEY_CHARDEV: &str = "chardev=";
const KEY_MODE: &str = "mode=";
const KEY_PRETTY: &str = "pretty=";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum ReadlineControl {
    Readline,
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
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Mon {
    #[builder(into)]
    chardev: ShellString,
    mode: Option<ReadlineControl>,
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
        mon.parse(s).map_err(|e| ShellStringError::from_parse(e))
    }
}

pco!(mode, alphanumeric1, ReadlineControl, KEY_MODE);
pco!(pretty, alphanumeric1, OnOff, KEY_PRETTY);

pub fn mon(s: &mut &str) -> ModalResult<Mon> {
    let _ = literal(KEY_CHARDEV).parse_next(s)?;
    let chardev = shell_string_until_comma.parse_to::<ShellString>().parse_next(s)?;
    let mode = opt(mode).parse_next(s)?;
    let pretty = opt(pretty).parse_next(s)?;
    Ok(Mon { chardev, mode, pretty })
}
