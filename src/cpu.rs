use std::ops::Add;

use bon::Builder;

use crate::ToCommand;
use crate::cpu_flags::CPUFlags;
use crate::cpu_type::CpuTypeX86_64;

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder)]
pub struct CpuX86 {
    cpu_type: CpuTypeX86_64,
    migratable: Option<bool>,
    flags: Vec<CPUFlags>,
}

impl CpuX86 {
    pub fn new(cpu_type: CpuTypeX86_64) -> Self {
        CpuX86 {
            cpu_type,
            migratable: None,
            flags: vec![],
        }
    }

    pub fn migratable(&mut self, state: bool) -> &mut Self {
        self.migratable = Some(state);
        self
    }
    pub fn flags(&mut self, flags: Vec<CPUFlags>) -> &mut Self {
        self.flags = flags;
        self
    }
}

impl ToCommand for CpuX86 {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec!["-cpu".to_string()];
        let mut ct = self.cpu_type.to_command().join("");

        if let Some(migratable) = self.migratable
            && migratable
        {
            ct.push_str(",migratable=yes");
        }
        let flags: Vec<String> = self
            .flags
            .iter()
            .map(|v| v.to_command())
            .collect::<Vec<Vec<String>>>()
            .concat();
        if !flags.is_empty() {
            let flags: Vec<String> = flags.iter().map(|flag| "-".to_string().add(flag)).collect();
            ct.push_str(format!(",{}", flags.join(",")).as_str());
        }

        cmd.push(ct);

        cmd
    }
}
