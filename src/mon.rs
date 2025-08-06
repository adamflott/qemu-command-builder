use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};
use bon::Builder;

pub enum ReadlineControl {
    Readline,
    Control,
}

impl ToArg for ReadlineControl {
    fn to_arg(&self) -> &str {
        match self {
            ReadlineControl::Readline => "readline",
            ReadlineControl::Control => "control",
        }
    }
}
#[derive(Builder)]
pub struct Mon {
    chardev: String,
    mode: Option<ReadlineControl>,
    pretty: Option<OnOff>,
}

impl ToCommand for Mon {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec!["-mon".to_string()];

        let mut args = vec![];
        args.push(format!("chardev={}", self.chardev));
        if let Some(mode) = &self.mode {
            args.push(format!("mode={}", mode.to_arg()));
        }
        if let Some(pretty) = &self.pretty {
            args.push(format!("pretty={}", pretty.to_arg()));
        }

        cmd.push(args.join(","));
        cmd
    }
}
