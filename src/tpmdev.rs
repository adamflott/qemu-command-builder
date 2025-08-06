use std::path::PathBuf;

use bon::Builder;

use crate::to_command::ToCommand;

#[derive(Default, Builder)]
pub struct Passthrough {
    id: String,
    path: Option<PathBuf>,
    cancel_path: Option<PathBuf>,
}

impl ToCommand for Passthrough {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        let mut args = vec![
            "passthrough".to_string(),
            format!("id={}", self.id.to_string()),
        ];

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

#[derive(Default, Builder)]
pub struct Emulator {
    id: String,
    chardev: String,
}

impl ToCommand for Emulator {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        let args = [
            "emulator".to_string(),
            format!("id={}", self.id),
            format!("chardev={}", self.chardev),
        ];

        cmd.push(args.join(","));
        cmd
    }
}

pub enum TpmDev {
    Passthrough(Passthrough),
    Emulator(Emulator),
}

impl ToCommand for TpmDev {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-tpmdev".to_string());

        match self {
            TpmDev::Passthrough(p) => cmd.append(&mut p.to_command()),
            TpmDev::Emulator(e) => cmd.append(&mut e.to_command()),
        }

        cmd
    }
}
