use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::to_command::ToArg;
use crate::to_command::ToCommand;

pub(crate) const ARG_COMPAT: &str = "-compat";

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
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct DeprecatedInput {
    deprecated_input: AcceptRejectCrash,
    deprecated_output: AcceptHide,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct UnstableInput {
    unstable_input: AcceptRejectCrash,
    unstable_output: AcceptHide,
}

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
                args
            }
            Compact::UnstableInput(unstable_input) => {
                let mut args = vec![];
                args.push(format!("unstable-input={}", unstable_input.unstable_input.to_arg()));
                args.push(format!("unstable-output={}", unstable_input.unstable_output.to_arg()));
                args
            }
        }
    }
}

impl FromStr for Compact {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
