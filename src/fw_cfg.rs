use bon::Builder;
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

use crate::to_command::ToCommand;

pub(crate) const ARG_FW_CFG: &str = "-fw_cfg";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum StringOrPathBuf {
    String(String),
    PathBuf(PathBuf),
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct FwCfg {
    name: Option<String>,
    data: Option<StringOrPathBuf>,
}

impl ToCommand for FwCfg {
    fn command(&self) -> String {
        ARG_FW_CFG.to_string()
    }
    fn to_args(&self) -> Vec<String> {
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

        args
    }
}

impl FromStr for FwCfg {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
