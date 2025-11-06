use bon::Builder;
use std::path::PathBuf;

use crate::to_command::ToCommand;

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum StringOrPathBuf {
    String(String),
    PathBuf(PathBuf),
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct FwCfg {
    name: Option<String>,
    data: Option<StringOrPathBuf>,
}

impl ToCommand for FwCfg {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-fw_cfg".to_string());

        let mut args = vec![];
        if let Some(name) = &self.name {
            args.push(format!("name={}", name));
        }
        if let Some(data) = &self.data {
            match &data {
                StringOrPathBuf::String(string) => {
                    args.push(format!("string={}", string));
                }
                StringOrPathBuf::PathBuf(path) => {
                    args.push(format!("file={}", path.display()));
                }
            }
        }
        cmd.push(args.join(","));
        cmd
    }
}
