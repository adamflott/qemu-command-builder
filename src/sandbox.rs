use crate::common::OnOff;
use crate::parsers::DELIM_COMMA;
use crate::to_command::{ToArg, ToCommand};
use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;

pub(crate) const ARG_SANDBOX: &str = "-sandbox";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum AllowDeny {
    Allow,
    Deny,
}

impl ToArg for AllowDeny {
    fn to_arg(&self) -> &str {
        match self {
            AllowDeny::Allow => "allow",
            AllowDeny::Deny => "deny",
        }
    }
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum AllowDenyChildren {
    Allow,
    Deny,
    Children,
}

impl ToArg for AllowDenyChildren {
    fn to_arg(&self) -> &str {
        match self {
            AllowDenyChildren::Allow => "allow",
            AllowDenyChildren::Deny => "deny",
            AllowDenyChildren::Children => "children",
        }
    }
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Sandbox {
    /// Whether seccomp sandboxing is enabled.
    mode: OnOff,
    /// Handling for obsolete syscalls.
    obsolete: Option<AllowDeny>,
    /// Privilege elevation policy.
    elevateprivileges: Option<AllowDenyChildren>,
    /// Process/thread spawning policy.
    spawn: Option<AllowDeny>,
    /// Resource control policy.
    resourcecontrol: Option<AllowDeny>,
}

impl ToCommand for Sandbox {
    fn command(&self) -> String {
        ARG_SANDBOX.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];
        args.push(self.mode.to_arg().to_string());

        if let Some(obsolete) = &self.obsolete {
            args.push(format!("obsolete={}", obsolete.to_arg()));
        }
        if let Some(elevateprivileges) = &self.elevateprivileges {
            args.push(format!("elevateprivileges={}", elevateprivileges.to_arg()))
        }
        if let Some(spawn) = &self.spawn {
            args.push(format!("spawn={}", spawn.to_arg()));
        }
        if let Some(resourcecontrol) = &self.resourcecontrol {
            args.push(format!("resourcecontrol={}", resourcecontrol.to_arg()));
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Sandbox {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(DELIM_COMMA);
        let mode = parts
            .next()
            .ok_or_else(|| "empty -sandbox argument".to_string())?
            .parse::<OnOff>()
            .map_err(|_| format!("invalid sandbox mode: {s}"))?;

        let mut obsolete = None;
        let mut elevateprivileges = None;
        let mut spawn = None;
        let mut resourcecontrol = None;

        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid -sandbox option: {part}"))?;
            match key {
                "obsolete" => {
                    obsolete = Some(match value {
                        "allow" => AllowDeny::Allow,
                        "deny" => AllowDeny::Deny,
                        _ => return Err(format!("invalid obsolete value: {value}")),
                    })
                }
                "elevateprivileges" => {
                    elevateprivileges = Some(match value {
                        "allow" => AllowDenyChildren::Allow,
                        "deny" => AllowDenyChildren::Deny,
                        "children" => AllowDenyChildren::Children,
                        _ => return Err(format!("invalid elevateprivileges value: {value}")),
                    })
                }
                "spawn" => {
                    spawn = Some(match value {
                        "allow" => AllowDeny::Allow,
                        "deny" => AllowDeny::Deny,
                        _ => return Err(format!("invalid spawn value: {value}")),
                    })
                }
                "resourcecontrol" => {
                    resourcecontrol = Some(match value {
                        "allow" => AllowDeny::Allow,
                        "deny" => AllowDeny::Deny,
                        _ => return Err(format!("invalid resourcecontrol value: {value}")),
                    })
                }
                other => return Err(format!("unsupported -sandbox option: {other}")),
            }
        }

        Ok(Self { mode, obsolete, elevateprivileges, spawn, resourcecontrol })
    }
}
