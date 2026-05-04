use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;
use winnow::ascii::alphanumeric1;
use winnow::combinator::{alt, opt, preceded, separated};
use winnow::prelude::*;
use winnow::token::{literal, take_while};

use crate::common::{OnOff, YesNo};
use crate::cpu_flags::CPUFlag;
use crate::cpu_type::{CpuTypeAarch64, CpuTypeX86_64};
use crate::parsers::{DELIM_COMMA, ascii_plus_more};
use crate::shell_string::ShellStringError;
use crate::{ToArg, ToCommand, qao};

pub(crate) const ARG_CPU: &str = "-cpu";

const KEY_MIGRATABLE: &str = "migratable=";

/// An x86 `-cpu` argument.
///
/// QEMU accepts a CPU model followed by comma-separated feature modifiers such
/// as `migratable=yes`, `vmx`, or `-svm`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct CpuX86 {
    cpu_type: CpuTypeX86_64,
    migratable: Option<YesNo>,
    flags: Option<BTreeMap<CPUFlag, OnOff>>,
}

impl CpuX86 {
    /// Creates a CPU argument for the given x86_64 CPU model.
    pub fn new(cpu_type: CpuTypeX86_64) -> Self {
        CpuX86 {
            cpu_type,
            migratable: None,
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
        cpu_x86_64.parse(s).map_err(|e| ShellStringError::from_parse(e))
    }
}

fn cpu_type(s: &mut &str) -> ModalResult<CpuTypeX86_64> {
    ascii_plus_more.parse_to::<CpuTypeX86_64>().parse_next(s)
}

fn migratable_item(s: &mut &str) -> ModalResult<YesNo> {
    let _ = literal(KEY_MIGRATABLE).parse_next(s)?;
    alphanumeric1.parse_to::<YesNo>().parse_next(s)
}

pub fn cpu_flag_parser<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1.., |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '.').parse_next(input)
}

fn cpu_flag(s: &mut &str) -> ModalResult<(CPUFlag, OnOff)> {
    let state = opt(literal('-')).parse_next(s)?;
    let flag = cpu_flag_parser.parse_to::<CPUFlag>().parse_next(s)?;
    Ok((
        flag,
        match state {
            None => OnOff::On,
            Some(_) => OnOff::Off,
        },
    ))
}

fn cpu_flag_property(s: &mut &str) -> ModalResult<(CPUFlag, OnOff)> {
    let flag = cpu_flag_parser.parse_to::<CPUFlag>().parse_next(s)?;
    let _ = literal("=").parse_next(s)?;
    let state = alphanumeric1.parse_to::<OnOff>().parse_next(s)?;
    Ok((flag, state))
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum CpuX86Item {
    Migratable(YesNo),
    Flag(CPUFlag, OnOff),
}

fn cpu_x86_item(s: &mut &str) -> ModalResult<CpuX86Item> {
    alt((
        migratable_item.map(CpuX86Item::Migratable),
        cpu_flag_property.map(|(flag, state)| CpuX86Item::Flag(flag, state)),
        cpu_flag.map(|(flag, state)| CpuX86Item::Flag(flag, state)),
    ))
    .parse_next(s)
}

fn cpu_x86_64(s: &mut &str) -> ModalResult<CpuX86> {
    let cpu_type = cpu_type(s)?;
    let items: Option<Vec<CpuX86Item>> = opt(preceded(literal(DELIM_COMMA), separated(1.., cpu_x86_item, DELIM_COMMA))).parse_next(s)?;
    let mut migratable = None;
    let mut flags = BTreeMap::new();

    if let Some(items) = items {
        for item in items {
            match item {
                CpuX86Item::Migratable(value) => migratable = Some(value),
                CpuX86Item::Flag(flag, state) => {
                    flags.insert(flag, state);
                }
            }
        }
    }

    let flags = (!flags.is_empty()).then_some(flags);
    Ok(CpuX86 { cpu_type, migratable, flags })
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
