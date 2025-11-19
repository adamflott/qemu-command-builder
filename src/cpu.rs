use std::collections::BTreeSet;
use std::ops::Add;
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;
use winnow::ascii::alphanumeric1;
use winnow::combinator::{opt, separated};
use winnow::prelude::*;
use winnow::token::literal;

use crate::common::YesNo;
use crate::cpu_flags::CPUFlag;
use crate::cpu_type::{CpuTypeAarch64, CpuTypeX86_64};
use crate::parsers::{DELIM_COMMA, ascii_plus_more};
use crate::shell_string::ShellStringError;
use crate::{ToArg, ToCommand, pco, qao};

pub(crate) const ARG_CPU: &str = "-cpu";

const KEY_MIGRATABLE: &str = "migratable=";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct CpuX86 {
    cpu_type: CpuTypeX86_64,
    migratable: Option<YesNo>,
    flags: Option<BTreeSet<CPUFlag>>,
}

impl CpuX86 {
    pub fn new(cpu_type: CpuTypeX86_64) -> Self {
        CpuX86 {
            cpu_type,
            migratable: None,
            flags: None,
        }
    }

    pub fn migratable(&mut self, state: YesNo) -> &mut Self {
        self.migratable = Some(state);
        self
    }
    pub fn flags(&mut self, flags: BTreeSet<CPUFlag>) -> &mut Self {
        self.flags = Some(flags);
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
            let flags: Vec<String> = flags.iter().map(|v| v.to_args()).collect::<Vec<Vec<String>>>().concat();
            if !flags.is_empty() {
                let flags: Vec<String> = flags.iter().map(|flag| "-".to_string().add(flag)).collect();
                args.push(format!("{}", flags.join(DELIM_COMMA)));
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

pco!(migratable, alphanumeric1, YesNo, KEY_MIGRATABLE);

fn cpu_type(s: &mut &str) -> ModalResult<CpuTypeX86_64> {
    Ok(ascii_plus_more.parse_to::<CpuTypeX86_64>().parse_next(s)?)
}

fn cpu_flag(s: &mut &str) -> ModalResult<CPUFlag> {
    Ok(alphanumeric1.parse_to::<CPUFlag>().parse_next(s)?)
}
fn cpu_flags(s: &mut &str) -> ModalResult<BTreeSet<CPUFlag>> {
    let flags: Vec<CPUFlag> = separated(1.., cpu_flag, DELIM_COMMA).parse_next(s)?;
    let flags = BTreeSet::from_iter(flags);
    Ok(flags)
}

fn cpu_x86_64(s: &mut &str) -> ModalResult<CpuX86> {
    let cpu_type = cpu_type(s)?;
    let migratable = opt(migratable).parse_next(s)?;
    let flags = opt(cpu_flags).parse_next(s)?;
    Ok(CpuX86 { cpu_type, migratable, flags })
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct CpuAarch64 {
    cpu_type: CpuTypeAarch64,
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
