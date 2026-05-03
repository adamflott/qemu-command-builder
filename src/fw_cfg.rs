use bon::Builder;
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

use crate::parsers::DELIM_COMMA;
use crate::to_command::ToCommand;

pub(crate) const ARG_FW_CFG: &str = "-fw_cfg";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum StringOrPathBuf {
    String(String),
    PathBuf(PathBuf),
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct FwCfg {
    /// The fw_cfg entry name.
    name: Option<String>,
    /// Either inline string contents or a file path.
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

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for FwCfg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(DELIM_COMMA);
        let first = parts.next().ok_or_else(|| "empty -fw_cfg argument".to_string())?;

        let mut name = None;
        let mut data = None;

        if let Some(value) = first.strip_prefix("name=") {
            name = Some(value.to_string());
        } else if !first.contains('=') {
            name = Some(first.to_string());
        } else {
            let (key, value) = first.split_once('=').ok_or_else(|| format!("invalid -fw_cfg option: {first}"))?;
            match key {
                "file" => data = Some(StringOrPathBuf::PathBuf(PathBuf::from(value))),
                "string" => data = Some(StringOrPathBuf::String(value.to_string())),
                other => return Err(format!("unsupported -fw_cfg option: {other}")),
            }
        }

        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid -fw_cfg option: {part}"))?;
            match key {
                "name" => name = Some(value.to_string()),
                "file" => data = Some(StringOrPathBuf::PathBuf(PathBuf::from(value))),
                "string" => data = Some(StringOrPathBuf::String(value.to_string())),
                other => return Err(format!("unsupported -fw_cfg option: {other}")),
            }
        }

        Ok(Self { name, data })
    }
}
