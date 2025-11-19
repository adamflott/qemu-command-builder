use crate::parsers::DELIM_COMMA;
use crate::shell_string::ShellStringError;
use crate::to_command::ToCommand;
use crate::{pco, qao};
use bon::Builder;
use proptest_derive::Arbitrary;
use std::fmt::Display;
use std::str::FromStr;
use winnow::ascii::{dec_uint, digit1};
use winnow::combinator::{fail, opt};
use winnow::token::{literal, one_of};
use winnow::{ModalResult, Parser};

pub(crate) const ARG_MEMORY: &str = "-m";

const KEY_SLOTS: &str = "slots=";
const KEY_MAXMEM: &str = "maxmem=";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum MemoryUnit {
    Bytes(u64),
    MegaBytes(u64),
    GigaBytes(u64),
}

impl Display for MemoryUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MemoryUnit::Bytes(amount) => {
                write!(f, "{}", amount)
            }
            MemoryUnit::MegaBytes(amount) => {
                write!(f, "{}M", amount)
            }
            MemoryUnit::GigaBytes(amount) => {
                write!(f, "{}G", amount)
            }
        }
    }
}

impl FromStr for MemoryUnit {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        memoryunit.parse(s).map_err(|e| ShellStringError::from_parse(e))
    }
}
/// Sets guest startup RAM size to megs megabytes. Default is 128 MiB.
/// Optionally, a suffix of "M" or "G" can be used to signify a value in
/// megabytes or gigabytes respectively. Optional pair slots, maxmem
/// could be used to set amount of hotpluggable memory slots and maximum
/// amount of memory. Note that maxmem must be aligned to the page size.
///
/// For example, the following command-line sets the guest startup RAM
/// size to 1GB, creates 3 slots to hotplug additional memory and sets
/// the maximum memory the guest can reach to 4GB:
///
/// `-m 1G,slots=3,maxmem=4G`
///
/// If slots and maxmem are not specified, memory hotplug won't be
/// enabled and the guest startup RAM will never increase.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Memory {
    mem: MemoryUnit,
    slots: Option<usize>,
    maxmem: Option<MemoryUnit>,
}

impl ToCommand for Memory {
    fn command(&self) -> String {
        ARG_MEMORY.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];

        args.push(self.mem.to_string());
        qao!(self.slots, args, KEY_SLOTS);
        qao!(&self.maxmem, args, KEY_MAXMEM);

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Memory {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        m.parse(s).map_err(|e| ShellStringError::from_parse(e))
    }
}

fn memoryunit(s: &mut &str) -> ModalResult<MemoryUnit> {
    let amount = dec_uint.parse_next(s)?;
    let unit = opt(one_of(['M', 'G'])).parse_next(s)?;
    match unit {
        None => Ok(MemoryUnit::Bytes(amount)),
        Some(c) => match c {
            'M' => Ok(MemoryUnit::MegaBytes(amount)),
            'G' => Ok(MemoryUnit::GigaBytes(amount)),
            _ => fail(s),
        },
    }
}

fn memoryunit_with_comma(s: &mut &str) -> ModalResult<MemoryUnit> {
    literal(DELIM_COMMA).parse_next(s)?;
    literal(KEY_MAXMEM).parse_next(s)?;
    memoryunit.parse_next(s)
}

pco!(slots, digit1, usize, KEY_SLOTS);

fn m(s: &mut &str) -> ModalResult<Memory> {
    let mem = memoryunit.parse_next(s)?;
    let slots = opt(slots).parse_next(s)?;
    let maxmem = opt(memoryunit_with_comma).parse_next(s)?;
    Ok(Memory { mem, slots, maxmem })
}
