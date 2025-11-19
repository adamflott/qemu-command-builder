#[cfg(test)]
mod test {
    use qemu_command_builder::common::YesNo;
    use qemu_command_builder::cpu::CpuX86;
    use qemu_command_builder::cpu_flags::CPUFlag;
    use qemu_command_builder::cpu_type::CpuTypeX86_64;
    use qemu_command_builder::to_command::ToCommand;
    use std::collections::BTreeSet;

    #[test]
    fn cpu() {
        let mut cpu = CpuX86::new(CpuTypeX86_64::Base);
        cpu.migratable(YesNo::No);
        cpu.flags(BTreeSet::from([CPUFlag::X3dnow, CPUFlag::Vmx]));

        assert_eq!(cpu.to_command(), vec!["-cpu".to_string(), "base,migratable=no,-3dnow,-vmx".to_string()]);
        assert_eq!(cpu.to_single_command(), "-cpu base,migratable=no,-3dnow,-vmx");
    }
}
