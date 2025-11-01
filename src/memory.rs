use bon::Builder;

use crate::to_command::ToCommand;

#[derive(Debug, Clone)]
pub enum MemoryUnit {
    Bytes(u64),
    MegaBytes(u64),
    GigaBytes(u64),
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
#[derive(Debug, Clone, Builder)]
pub struct Memory {
    mem: MemoryUnit,
    slots: Option<usize>,
    maxmem: Option<MemoryUnit>,
}

impl ToCommand for Memory {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-m".to_string());

        let mut arg = String::new();
        match &self.mem {
            MemoryUnit::Bytes(amount) => {
                arg.push_str(format!("{}", amount).as_str());
            }
            MemoryUnit::MegaBytes(amount) => {
                arg.push_str(format!("{}M", amount).as_str());
            }
            MemoryUnit::GigaBytes(amount) => {
                arg.push_str(format!("{}G", amount).as_str());
            }
        }
        if let Some(slots) = self.slots {
            arg.push_str(format!(",slots={}", slots).as_str());
        }
        if let Some(maxmem) = &self.maxmem {
            match maxmem {
                MemoryUnit::Bytes(amount) => {
                    arg.push_str(format!(",maxmem={}", amount).as_str());
                }
                MemoryUnit::MegaBytes(amount) => {
                    arg.push_str(format!(",maxmem={}M", amount).as_str());
                }
                MemoryUnit::GigaBytes(amount) => {
                    arg.push_str(format!(",maxmem={}G", amount).as_str());
                }
            }
        }
        cmd.push(arg);

        cmd
    }
}
