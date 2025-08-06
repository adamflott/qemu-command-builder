use crate::to_command::ToArg;
use crate::to_command::ToCommand;
use bon::Builder;

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
#[derive(Builder)]
pub struct DeprecatedInput {
    deprecated_input: AcceptRejectCrash,
    deprecated_output: AcceptHide,
}

#[derive(Builder)]
pub struct UnstableInput {
    unstable_input: AcceptRejectCrash,
    unstable_output: AcceptHide,
}

pub enum Compact {
    DeprecatedInput(DeprecatedInput),
    UnstableInput(UnstableInput),
}

impl ToCommand for Compact {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-compact".to_string());

        match self {
            Compact::DeprecatedInput(deprecated_input) => {
                let mut args = vec![];
                args.push(format!(
                    "deprecated-input={}",
                    deprecated_input.deprecated_input.to_arg()
                ));
                args.push(format!(
                    "deprecated-output={}",
                    deprecated_input.deprecated_output.to_arg()
                ));
                cmd.push(args.join(","));
            }
            Compact::UnstableInput(unstable_input) => {
                let mut args = vec![];
                args.push(format!(
                    "unstable-input={}",
                    unstable_input.unstable_input.to_arg()
                ));
                args.push(format!(
                    "unstable-output={}",
                    unstable_input.unstable_output.to_arg()
                ));
                cmd.push(args.join(","));
            }
        }
        cmd
    }
}
