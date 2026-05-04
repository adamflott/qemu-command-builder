use bon::Builder;
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

use crate::parsers::DELIM_COMMA;
use crate::to_command::ToCommand;

pub(crate) const ARG_TPMDEV: &str = "-tpmdev";

/// A `-tpmdev passthrough,...` backend.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Passthrough {
    id: String,
    path: Option<PathBuf>,
    cancel_path: Option<PathBuf>,
}

impl ToCommand for Passthrough {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["passthrough".to_string(), format!("id={}", self.id.to_string())];

        if let Some(path) = &self.path {
            args.push(format!("path={}", path.display()));
        }
        if let Some(cancel_path) = &self.cancel_path {
            args.push(format!("cancel-path={}", cancel_path.display()));
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Passthrough {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(DELIM_COMMA);
        let backend = parts.next().ok_or_else(|| "empty -tpmdev argument".to_string())?;
        if backend != "passthrough" {
            return Err(format!("expected passthrough backend, got {backend}"));
        }

        let mut id = None;
        let mut path = None;
        let mut cancel_path = None;
        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid passthrough option: {part}"))?;
            match key {
                "id" => id = Some(value.to_string()),
                "path" => path = Some(PathBuf::from(value)),
                "cancel-path" => cancel_path = Some(PathBuf::from(value)),
                other => return Err(format!("unsupported passthrough option: {other}")),
            }
        }
        Ok(Self {
            id: id.ok_or_else(|| "passthrough requires id=".to_string())?,
            path,
            cancel_path,
        })
    }
}

/// A `-tpmdev emulator,...` backend.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Emulator {
    id: String,
    chardev: String,
}

impl ToCommand for Emulator {
    fn to_args(&self) -> Vec<String> {
        vec![["emulator".to_string(), format!("id={}", self.id), format!("chardev={}", self.chardev)].join(DELIM_COMMA)]
    }
}

impl FromStr for Emulator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(DELIM_COMMA);
        let backend = parts.next().ok_or_else(|| "empty -tpmdev argument".to_string())?;
        if backend != "emulator" {
            return Err(format!("expected emulator backend, got {backend}"));
        }

        let mut id = None;
        let mut chardev = None;
        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid emulator option: {part}"))?;
            match key {
                "id" => id = Some(value.to_string()),
                "chardev" => chardev = Some(value.to_string()),
                other => return Err(format!("unsupported emulator option: {other}")),
            }
        }

        Ok(Self {
            id: id.ok_or_else(|| "emulator requires id=".to_string())?,
            chardev: chardev.ok_or_else(|| "emulator requires chardev=".to_string())?,
        })
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
        match self {
            TpmDev::Passthrough(p) => p.to_args(),
            TpmDev::Emulator(e) => e.to_args(),
        }
    }
}

impl FromStr for TpmDev {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("passthrough,") || s == "passthrough" {
            return Ok(Self::Passthrough(s.parse::<Passthrough>()?));
        }
        if s.starts_with("emulator,") || s == "emulator" {
            return Ok(Self::Emulator(s.parse::<Emulator>()?));
        }
        Err(format!("unsupported tpmdev backend: {s}"))
    }
}
