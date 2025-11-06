use bon::Builder;
use std::path::PathBuf;

use crate::to_command::ToCommand;

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct Trace {
    enable: Option<String>,
    events: Option<PathBuf>,
    file: Option<PathBuf>,
}

impl ToCommand for Trace {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-trace".to_string());

        let mut args = vec![];

        if let Some(enable) = &self.enable {
            args.push(format!("enable={}", enable));
        }
        if let Some(events) = &self.events {
            args.push(format!("events={}", events.display()));
        }
        if let Some(file) = &self.file {
            args.push(format!("file={}", file.display()));
        }
        cmd.push(args.join(","));
        cmd
    }
}
