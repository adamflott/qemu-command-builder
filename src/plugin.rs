use bon::Builder;
use std::path::PathBuf;

use crate::to_command::ToCommand;

#[derive(Default, Builder)]
pub struct Plugin {
    file: Option<PathBuf>,
    args: Option<Vec<(String, String)>>,
}

impl ToCommand for Plugin {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-plugin".to_string());

        let mut args = vec![];

        if let Some(file) = &self.file {
            args.push(format!("file={}", file.display()));
        }
        if let Some(argss) = &self.args {
            for arg in argss {
                args.push(format!("{}={}", arg.0, arg.1));
            }
        }
        cmd.push(args.join(","));
        cmd
    }
}
