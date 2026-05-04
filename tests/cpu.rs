#[cfg(test)]
mod test {
    use qemu_command_builder::common::{OnOff, YesNo};
    use qemu_command_builder::cpu::CpuX86;
    use qemu_command_builder::cpu_flags::CPUFlag;
    use qemu_command_builder::cpu_type::CpuTypeX86_64;
    use qemu_command_builder::to_command::ToCommand;
    use std::collections::BTreeSet;
    use std::str::FromStr;

    #[test]
    fn cpu_displays_enabled_and_disabled_flags() {
        let mut cpu = CpuX86::new(CpuTypeX86_64::Base);
        cpu.migratable(YesNo::No);
        cpu.flags(BTreeSet::from([(CPUFlag::X3dnow, OnOff::Off), (CPUFlag::Vmx, OnOff::On)]));

        assert_eq!(cpu.to_command(), vec!["-cpu".to_string(), "base,migratable=no,-3dnow,vmx".to_string()]);
        assert_eq!(cpu.to_single_command(), "-cpu base,migratable=no,-3dnow,vmx");
    }

    #[test]
    fn cpu_parses_mixed_feature_states() {
        let parsed = CpuX86::from_str("host,migratable=yes,-svm,vmx,sse4.1").unwrap();

        let mut expected = CpuX86::new(CpuTypeX86_64::Host);
        expected.migratable(YesNo::Yes);
        expected.flags(BTreeSet::from([(CPUFlag::Svm, OnOff::Off), (CPUFlag::Vmx, OnOff::On), (CPUFlag::Sse4_1, OnOff::On)]));

        assert_eq!(parsed, expected);
        assert_eq!(parsed.to_command(), vec!["-cpu".to_string(), "host,migratable=yes,sse4.1,-svm,vmx".to_string()]);
    }

    #[test]
    fn cpu_parses_property_style_feature_states() {
        let parsed = CpuX86::from_str("host,migratable=yes,-vmx,-svm,invpcid=off").unwrap();

        let mut expected = CpuX86::new(CpuTypeX86_64::Host);
        expected.migratable(YesNo::Yes);
        expected.flags(BTreeSet::from([(CPUFlag::Vmx, OnOff::Off), (CPUFlag::Svm, OnOff::Off), (CPUFlag::Invpcid, OnOff::Off)]));

        assert_eq!(parsed, expected);
        assert_eq!(parsed.to_command(), vec!["-cpu".to_string(), "host,migratable=yes,-invpcid,-svm,-vmx".to_string()]);
    }

    #[test]
    fn cpu_parses_properties_and_flags_in_any_order_after_model() {
        let parsed = CpuX86::from_str("base,vmx,migratable=no,-3dnow").unwrap();

        let mut expected = CpuX86::new(CpuTypeX86_64::Base);
        expected.migratable(YesNo::No);
        expected.flags(BTreeSet::from([(CPUFlag::Vmx, OnOff::On), (CPUFlag::X3dnow, OnOff::Off)]));

        assert_eq!(parsed, expected);
    }

    #[test]
    fn cpu_duplicate_feature_modifiers_use_last_state() {
        let parsed = CpuX86::from_str("host,vmx,-vmx,vmx,-svm,svm").unwrap();

        let mut expected = CpuX86::new(CpuTypeX86_64::Host);
        expected.flags(BTreeSet::from([(CPUFlag::Vmx, OnOff::On), (CPUFlag::Svm, OnOff::On)]));

        assert_eq!(parsed, expected);
        assert_eq!(parsed.to_command(), vec!["-cpu".to_string(), "host,svm,vmx".to_string()]);
    }
}
