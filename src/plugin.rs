use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

pub(crate) const ARG_PLUGIN: &str = "-plugin";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Plugin {
    file: Option<PathBuf>,
    args: Option<Vec<(String, String)>>,
}

impl ToCommand for Plugin {
    fn command(&self) -> String {
        ARG_PLUGIN.to_string()
    }

    fn to_args(&self) -> Vec<String> {
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

impl FromStr for Plugin {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
