use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::parsers::DELIM_COMMA;
use crate::to_command::ToArg;
use crate::to_command::ToCommand;

pub(crate) const ARG_COMPAT: &str = "-compat";

/// Input policy values for `-compat` compatibility knobs.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum AcceptRejectCrash {
    Accept,
    Reject,
    Crash,
}

impl ToArg for AcceptRejectCrash {
    fn to_arg(&self) -> &str {
        match self {
            AcceptRejectCrash::Accept => "accept",
            AcceptRejectCrash::Reject => "reject",
            AcceptRejectCrash::Crash => "crash",
        }
    }
}

/// Output policy values for `-compat` compatibility knobs.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum AcceptHide {
    Accept,
    Hide,
}
impl ToArg for AcceptHide {
    fn to_arg(&self) -> &str {
        match self {
            AcceptHide::Accept => "accept",
            AcceptHide::Hide => "hide",
        }
    }
}
/// `-compat deprecated-input=...,deprecated-output=...`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct DeprecatedInput {
    deprecated_input: AcceptRejectCrash,
    deprecated_output: AcceptHide,
}

/// `-compat unstable-input=...,unstable-output=...`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct UnstableInput {
    unstable_input: AcceptRejectCrash,
    unstable_output: AcceptHide,
}

/// A QEMU `-compat` definition.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum Compact {
    DeprecatedInput(DeprecatedInput),
    UnstableInput(UnstableInput),
}

impl ToCommand for Compact {
    fn command(&self) -> String {
        ARG_COMPAT.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        match self {
            Compact::DeprecatedInput(deprecated_input) => {
                let mut args = vec![];
                args.push(format!("deprecated-input={}", deprecated_input.deprecated_input.to_arg()));
                args.push(format!("deprecated-output={}", deprecated_input.deprecated_output.to_arg()));
                vec![args.join(DELIM_COMMA)]
            }
            Compact::UnstableInput(unstable_input) => {
                let mut args = vec![];
                args.push(format!("unstable-input={}", unstable_input.unstable_input.to_arg()));
                args.push(format!("unstable-output={}", unstable_input.unstable_output.to_arg()));
                vec![args.join(DELIM_COMMA)]
            }
        }
    }
}

impl FromStr for Compact {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut deprecated_input = None;
        let mut deprecated_output = None;
        let mut unstable_input = None;
        let mut unstable_output = None;

        for part in s.split(DELIM_COMMA).filter(|part| !part.is_empty()) {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid -compat option: {part}"))?;
            match key {
                "deprecated-input" => {
                    deprecated_input = Some(parse_accept_reject_crash(value)?);
                }
                "deprecated-output" => {
                    deprecated_output = Some(parse_accept_hide(value)?);
                }
                "unstable-input" => {
                    unstable_input = Some(parse_accept_reject_crash(value)?);
                }
                "unstable-output" => {
                    unstable_output = Some(parse_accept_hide(value)?);
                }
                other => return Err(format!("unsupported -compat option: {other}")),
            }
        }

        match (deprecated_input, deprecated_output, unstable_input, unstable_output) {
            (Some(input), Some(output), None, None) => Ok(Self::DeprecatedInput(DeprecatedInput {
                deprecated_input: input,
                deprecated_output: output,
            })),
            (None, None, Some(input), Some(output)) => Ok(Self::UnstableInput(UnstableInput {
                unstable_input: input,
                unstable_output: output,
            })),
            (Some(_), None, None, None) | (None, Some(_), None, None) => Err("both deprecated-input and deprecated-output are required together".to_string()),
            (None, None, Some(_), None) | (None, None, None, Some(_)) => Err("both unstable-input and unstable-output are required together".to_string()),
            (None, None, None, None) => Err("empty -compat argument".to_string()),
            _ => Err("cannot mix deprecated-* and unstable-* compat policies in one Compact value".to_string()),
        }
    }
}

fn parse_accept_reject_crash(value: &str) -> Result<AcceptRejectCrash, String> {
    match value {
        "accept" => Ok(AcceptRejectCrash::Accept),
        "reject" => Ok(AcceptRejectCrash::Reject),
        "crash" => Ok(AcceptRejectCrash::Crash),
        _ => Err(format!("invalid compat input policy: {value}")),
    }
}

fn parse_accept_hide(value: &str) -> Result<AcceptHide, String> {
    match value {
        "accept" => Ok(AcceptHide::Accept),
        "hide" => Ok(AcceptHide::Hide),
        _ => Err(format!("invalid compat output policy: {value}")),
    }
}
