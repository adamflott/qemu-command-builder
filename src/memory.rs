use crate::parsers::DELIM_COMMA;
use crate::qao;
use crate::shell_string::ShellStringError;
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::fmt::Display;
use std::str::FromStr;

pub(crate) const ARG_MEMORY: &str = "-m";

const KEY_SIZE: &str = "size=";
const KEY_SLOTS: &str = "slots=";
const KEY_MAXMEM: &str = "maxmem=";

/// Memory quantities accepted by QEMU `-m`.
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
        parse_memory_unit(s).map_err(ShellStringError::new)
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
    /// Initial guest RAM size.
    mem: MemoryUnit,
    /// Number of hotpluggable memory slots.
    slots: Option<usize>,
    /// Maximum guest RAM size.
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
        parse_memory(s).map_err(ShellStringError::new)
    }
}

fn parse_memory_unit(s: &str) -> Result<MemoryUnit, String> {
    if let Some(amount) = s.strip_suffix('M') {
        return Ok(MemoryUnit::MegaBytes(amount.parse::<u64>().map_err(|e| format!("invalid memory amount: {e}"))?));
    }
    if let Some(amount) = s.strip_suffix('G') {
        return Ok(MemoryUnit::GigaBytes(amount.parse::<u64>().map_err(|e| format!("invalid memory amount: {e}"))?));
    }

    Ok(MemoryUnit::Bytes(s.parse::<u64>().map_err(|e| format!("invalid memory amount: {e}"))?))
}

fn parse_memory(s: &str) -> Result<Memory, String> {
    let mut parts = s.split(DELIM_COMMA);
    let first = parts.next().ok_or_else(|| "empty memory argument".to_string())?;

    let mem = if let Some(value) = first.strip_prefix(KEY_SIZE) {
        parse_memory_unit(value)?
    } else if first.contains('=') {
        return Err(format!("unsupported memory option: {first}"));
    } else {
        parse_memory_unit(first)?
    };

    let mut slots = None;
    let mut maxmem = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid memory option: {part}"))?;
        match key {
            "size" => return Err("size= is only valid as the first -m component".to_string()),
            "slots" => {
                slots = Some(value.parse::<usize>().map_err(|e| format!("invalid slots value: {e}"))?);
            }
            "maxmem" => maxmem = Some(parse_memory_unit(value)?),
            other => return Err(format!("unsupported memory option: {other}")),
        }
    }

    Ok(Memory { mem, slots, maxmem })
}
