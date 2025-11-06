use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};
use bon::Builder;

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
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
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
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
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder)]
pub struct Sandbox {
    mode: OnOff,
    obsolete: Option<AllowDeny>,
    elevateprivileges: Option<AllowDenyChildren>,
    spawn: Option<AllowDeny>,
    resourcecontrol: Option<AllowDeny>,
}

impl ToCommand for Sandbox {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec!["-sandbox".to_string()];

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

        cmd.push(args.join(","));
        cmd
    }
}
