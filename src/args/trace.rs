use crate::parsers::{ARG_TRACE, DELIM_COMMA};
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

/// A QEMU `-trace [[enable=]pattern][,events=file][,file=file]` definition.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Trace {
    /// An enabled trace event pattern.
    enable: Option<String>,
    /// The events file.
    events: Option<PathBuf>,
    /// The trace output file.
    file: Option<PathBuf>,
}

impl ToCommand for Trace {
    fn command(&self) -> String {
        ARG_TRACE.to_string()
    }
    fn to_args(&self) -> Vec<String> {
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
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Trace {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut enable = None;
        let mut events = None;
        let mut file = None;

        let mut parts = s.split(DELIM_COMMA);
        if let Some(first) = parts.next()
            && !first.is_empty()
        {
            if let Some(value) = first.strip_prefix("enable=") {
                enable = Some(value.to_string());
            } else if first.contains('=') {
                let (key, value) = first.split_once('=').ok_or_else(|| format!("invalid -trace option: {first}"))?;
                match key {
                    "events" => events = Some(PathBuf::from(value)),
                    "file" => file = Some(PathBuf::from(value)),
                    other => return Err(format!("unsupported -trace option: {other}")),
                }
            } else {
                enable = Some(first.to_string());
            }
        }

        for part in parts {
            if !part.contains('=') {
                if enable.is_none() {
                    enable = Some(part.to_string());
                    continue;
                }
                return Err(format!("invalid -trace option: {part}"));
            }

            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid -trace option: {part}"))?;
            match key {
                "enable" => enable = Some(value.to_string()),
                "events" => events = Some(PathBuf::from(value)),
                "file" => file = Some(PathBuf::from(value)),
                other => return Err(format!("unsupported -trace option: {other}")),
            }
        }

        Ok(Self { enable, events, file })
    }
}
