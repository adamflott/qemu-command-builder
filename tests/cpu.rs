#[cfg(test)]
mod test {
    use qemu_command_builder::cpu::CpuX86;
    use qemu_command_builder::cpu_flags::CPUFlags;
    use qemu_command_builder::cpu_type::CpuTypeX86_64;
    use qemu_command_builder::to_command::ToCommand;

    #[test]
    fn cpu() {
        let mut cpu = CpuX86::new(CpuTypeX86_64::Base);
        cpu.migratable(false);
        cpu.flags(vec![CPUFlags::X3dnow, CPUFlags::Vmx]);

        assert_eq!(
            cpu.to_command(),
            vec!["-cpu".to_string(), "base,-3dnow,-vmx".to_string()]
        );
        assert_eq!(cpu.to_single_command(), "-cpu base,-3dnow,-vmx");
    }
}
