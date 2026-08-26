use crate::parsers::ARG_CPU;
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;
use winnow::prelude::*;
use winnow::token::take_while;

use crate::args::cpu_flags::CPUFlag;
use crate::args::cpu_type::{CpuTypeAarch64, CpuTypeX86_64};
use crate::common::{OnOff, YesNo};
use crate::parsers::{DELIM_COMMA, ascii_plus_more};
use crate::shell_string::ShellStringError;
use crate::{ToArg, ToCommand, qao};

const KEY_MIGRATABLE: &str = "migratable=";

/// An x86 `-cpu` argument.
///
/// QEMU accepts a CPU model followed by comma-separated feature modifiers such
/// as `migratable=yes`, `vmx`, or `-svm`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct CpuX86 {
    cpu_type: CpuTypeX86_64,
    migratable: Option<YesNo>,
    properties: Option<Vec<(String, String)>>,
    flags: Option<BTreeMap<CPUFlag, OnOff>>,
}

impl CpuX86 {
    /// Creates a CPU argument for the given x86_64 CPU model.
    pub fn new(cpu_type: CpuTypeX86_64) -> Self {
        CpuX86 {
            cpu_type,
            migratable: None,
            properties: None,
            flags: None,
        }
    }

    /// Sets the `migratable=` CPU property.
    pub fn migratable(&mut self, state: YesNo) -> &mut Self {
        self.migratable = Some(state);
        self
    }

    /// Sets CPU feature toggles.
    ///
    /// Enabled features are rendered as `flag`, while disabled features are
    /// rendered as `-flag`. If the same feature appears more than once, the
    /// final state for that feature wins.
    pub fn flags(&mut self, flags: BTreeSet<(CPUFlag, OnOff)>) -> &mut Self {
        let mut normalized = BTreeMap::new();
        for (flag, state) in flags {
            normalized.insert(flag, state);
        }
        self.flags = Some(normalized);
        self
    }
}

impl ToCommand for CpuX86 {
    fn command(&self) -> String {
        ARG_CPU.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut args = vec![self.cpu_type.to_arg().to_string()];

        qao!(&self.migratable, args, KEY_MIGRATABLE);
        if let Some(properties) = &self.properties {
            args.extend(properties.iter().map(|(key, value)| format!("{key}={value}")));
        }
        if let Some(flags) = &self.flags {
            let flags: Vec<String> = flags
                .iter()
                .map(|(flag, state)| match state {
                    OnOff::On => flag.to_arg().to_string(),
                    OnOff::Off => format!("-{}", flag.to_arg()),
                })
                .collect();
            if !flags.is_empty() {
                args.push(flags.join(DELIM_COMMA));
            }
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for CpuX86 {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_cpu_x86(s).map_err(ShellStringError::new)
    }
}

fn parse_cpu_x86(s: &str) -> Result<CpuX86, String> {
    let mut parts = s.split(DELIM_COMMA);
    let cpu_type = parts
        .next()
        .ok_or_else(|| "CPU model is required".to_string())?
        .parse::<CpuTypeX86_64>()
        .map_err(|_| "invalid CPU model".to_string())?;
    let mut migratable = None;
    let mut properties = Vec::new();
    let mut flags = BTreeMap::new();

    for part in parts {
        if let Some(value) = part.strip_prefix(KEY_MIGRATABLE) {
            migratable = Some(match value {
                "yes" | "on" => YesNo::Yes,
                "no" | "off" => YesNo::No,
                _ => return Err(format!("invalid migratable value: {value}")),
            });
        } else if let Some((key, value)) = part.split_once('=') {
            match (key.parse::<CPUFlag>(), value.parse::<OnOff>()) {
                (Ok(flag), Ok(state)) => {
                    flags.insert(flag, state);
                }
                _ => properties.push((key.to_string(), value.to_string())),
            }
        } else {
            let (name, state) = part.strip_prefix('-').map_or((part, OnOff::On), |name| (name, OnOff::Off));
            let flag = name.parse::<CPUFlag>().map_err(|_| format!("unsupported CPU feature: {name}"))?;
            flags.insert(flag, state);
        }
    }

    Ok(CpuX86 {
        cpu_type,
        migratable,
        properties: (!properties.is_empty()).then_some(properties),
        flags: (!flags.is_empty()).then_some(flags),
    })
}

pub fn cpu_flag_parser<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1.., |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '.').parse_next(input)
}

/// An aarch64 `-cpu` argument.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct CpuAarch64 {
    pub cpu_type: CpuTypeAarch64,
}

impl ToCommand for CpuAarch64 {
    fn command(&self) -> String {
        ARG_CPU.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        vec![self.cpu_type.to_command().join("")]
    }
}

fn cpu_type_aarch64(s: &mut &str) -> ModalResult<CpuTypeAarch64> {
    ascii_plus_more.parse_to::<CpuTypeAarch64>().parse_next(s)
}

impl FromStr for CpuAarch64 {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        cpu_aarch64.parse(s).map_err(|e| ShellStringError::from_parse(e))
    }
}

fn cpu_aarch64(s: &mut &str) -> ModalResult<CpuAarch64> {
    let cpu_type = cpu_type_aarch64.parse_next(s)?;
    Ok(CpuAarch64 { cpu_type })
}
