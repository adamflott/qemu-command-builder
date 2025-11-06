use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};
use bon::Builder;

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct Msg {
    timestamp: Option<OnOff>,
    guest_name: Option<OnOff>,
}

impl ToCommand for Msg {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-msg".to_string());

        let mut args = vec![];
        if let Some(timestamp) = &self.timestamp {
            args.push(format!("timestamp={}", timestamp.to_arg()));
        }
        if let Some(guest_name) = &self.guest_name {
            args.push(format!("guest-name={}", guest_name.to_arg()));
        }

        cmd.push(args.join(","));
        cmd
    }
}
