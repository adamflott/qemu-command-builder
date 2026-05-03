use bon::Builder;
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

use crate::to_command::ToCommand;

pub(crate) const ARG_TPMDEV: &str = "-tpmdev";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Passthrough {
    id: String,
    path: Option<PathBuf>,
    cancel_path: Option<PathBuf>,
}

impl ToCommand for Passthrough {
    fn to_args(&self) -> Vec<String> {
        let mut cmd = vec![];

        let mut args = vec!["passthrough".to_string(), format!("id={}", self.id.to_string())];

        if let Some(path) = &self.path {
            args.push(format!("path={}", path.display()));
        }
        if let Some(cancel_path) = &self.cancel_path {
            args.push(format!("cancel-path={}", cancel_path.display()));
        }

        cmd.push(args.join(","));
        cmd
    }
}

impl FromStr for Passthrough {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Emulator {
    id: String,
    chardev: String,
}

impl ToCommand for Emulator {
    fn to_args(&self) -> Vec<String> {
        let args = vec!["emulator".to_string(), format!("id={}", self.id), format!("chardev={}", self.chardev)];

        args
    }
}

impl FromStr for Emulator {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum TpmDev {
    Passthrough(Passthrough),
    Emulator(Emulator),
}

impl ToCommand for TpmDev {
    fn command(&self) -> String {
        ARG_TPMDEV.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];

        match self {
            TpmDev::Passthrough(p) => args.append(&mut p.to_command()),
            TpmDev::Emulator(e) => args.append(&mut e.to_command()),
        }

        args
    }
}

impl FromStr for TpmDev {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
