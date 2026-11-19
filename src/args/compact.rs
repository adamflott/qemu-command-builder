use crate::parsers::ARG_COMPAT;
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::parsers::DELIM_COMMA;
use crate::to_command::ToArg;
use crate::to_command::ToCommand;

/// Input policy values for `-compat` compatibility knobs.
///
/// The type name is retained for compatibility; QEMU 11.2 also accepts `warn`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum AcceptRejectCrash {
    Accept,
    Reject,
    Crash,
    /// Accept the input and issue repeated warnings on stderr.
    Warn,
}

impl ToArg for AcceptRejectCrash {
    fn to_arg(&self) -> &str {
        match self {
            AcceptRejectCrash::Accept => "accept",
            AcceptRejectCrash::Reject => "reject",
            AcceptRejectCrash::Crash => "crash",
            AcceptRejectCrash::Warn => "warn",
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

/// Independently selectable `-compat` policies, including QEMU 11.2's
/// `insecure-input`. Omitted policies retain QEMU's defaults.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CompatPolicies {
    deprecated_input: Option<AcceptRejectCrash>,
    deprecated_output: Option<AcceptHide>,
    unstable_input: Option<AcceptRejectCrash>,
    unstable_output: Option<AcceptHide>,
    /// Controls explicitly created device, machine and accelerator QOM types
    /// that do not provide a security boundary for untrusted input.
    insecure_input: Option<AcceptRejectCrash>,
}

/// A QEMU `-compat` definition.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum Compact {
    DeprecatedInput(DeprecatedInput),
    UnstableInput(UnstableInput),
    Policies(CompatPolicies),
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
            Compact::Policies(policies) => {
                let mut args = Vec::new();
                for (key, policy) in [
                    ("deprecated-input", policies.deprecated_input.as_ref().map(ToArg::to_arg)),
                    ("deprecated-output", policies.deprecated_output.as_ref().map(ToArg::to_arg)),
                    ("unstable-input", policies.unstable_input.as_ref().map(ToArg::to_arg)),
                    ("unstable-output", policies.unstable_output.as_ref().map(ToArg::to_arg)),
                    ("insecure-input", policies.insecure_input.as_ref().map(ToArg::to_arg)),
                ] {
                    if let Some(policy) = policy {
                        args.push(format!("{key}={policy}"));
                    }
                }
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
        let mut insecure_input = None;

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
                "insecure-input" => {
                    insecure_input = Some(parse_accept_reject_crash(value)?);
                }
                other => return Err(format!("unsupported -compat option: {other}")),
            }
        }

        match (deprecated_input, deprecated_output, unstable_input, unstable_output, insecure_input) {
            (Some(input), Some(output), None, None, None) => Ok(Self::DeprecatedInput(DeprecatedInput {
                deprecated_input: input,
                deprecated_output: output,
            })),
            (None, None, Some(input), Some(output), None) => Ok(Self::UnstableInput(UnstableInput {
                unstable_input: input,
                unstable_output: output,
            })),
            (None, None, None, None, None) => Err("empty -compat argument".to_string()),
            (deprecated_input, deprecated_output, unstable_input, unstable_output, insecure_input) => Ok(Self::Policies(CompatPolicies {
                deprecated_input,
                deprecated_output,
                unstable_input,
                unstable_output,
                insecure_input,
            })),
        }
    }
}

fn parse_accept_reject_crash(value: &str) -> Result<AcceptRejectCrash, String> {
    match value {
        "accept" => Ok(AcceptRejectCrash::Accept),
        "reject" => Ok(AcceptRejectCrash::Reject),
        "crash" => Ok(AcceptRejectCrash::Crash),
        "warn" => Ok(AcceptRejectCrash::Warn),
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
