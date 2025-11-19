use crate::common::OnOff;
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
    mode: OnOff,
    obsolete: Option<AllowDeny>,
    elevateprivileges: Option<AllowDenyChildren>,
    spawn: Option<AllowDeny>,
    resourcecontrol: Option<AllowDeny>,
}

impl ToCommand for Sandbox {
    fn command(&self) -> String {
        ARG_SANDBOX.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];
        match &self.mode {
            OnOff::On => {
                args.push("on".to_string());
            }
            OnOff::Off => {
                args.push("off".to_string());
            }
        }

        if let Some(obsolete) = &self.obsolete {
            args.push(obsolete.to_arg().to_string());
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

        args
    }
}

impl FromStr for Sandbox {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
