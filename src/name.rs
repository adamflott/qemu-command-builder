use bon::Builder;

use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};

/// Sets the name of the guest. This name will be displayed in the SDL
/// window caption. The name will also be used for the VNC server. Also
/// optionally set the top visible process name in Linux. Naming of
/// individual threads can also be enabled on Linux to aid debugging.
#[derive(Builder)]
pub struct Name {
    name: String,
    process: Option<String>,
    debug_threads: Option<OnOff>,
}

impl ToCommand for Name {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-name".to_string());

        let mut args = self.name.to_string();

        if let Some(process) = &self.process {
            args.push_str(format!(",process={}", process).as_str());
        }
        if let Some(debug_threads) = &self.debug_threads {
            args.push_str(format!(",debug-threads={}", debug_threads.to_arg()).as_str());
        }

        cmd.push(args);

        cmd
    }
}
