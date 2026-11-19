use crate::parsers::ARG_MACHINE;
use crate::shell_string::ShellString;
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;

/// A Hexagon machine, such as `V68N_1024` or `virt`, and its board properties.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct MachineHexagon {
    #[builder(into)]
    machine_type: ShellString,
    #[builder(default)]
    properties: Vec<(ShellString, ShellString)>,
}

impl ToCommand for MachineHexagon {
    fn command(&self) -> String {
        ARG_MACHINE.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut parts = vec![self.machine_type.as_ref().to_string()];
        parts.extend(self.properties.iter().map(|(key, value)| format!("{}={}", key.as_ref(), value.as_ref())));
        vec![parts.join(",")]
    }
}

impl FromStr for MachineHexagon {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut machine_type = None;
        let mut properties = Vec::new();
        for (position, part) in value.split(',').enumerate() {
            if position == 0 && !part.contains('=') {
                machine_type = Some(ShellString::new(part));
                continue;
            }
            let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid Hexagon machine option: {part}"))?;
            if key == "type" {
                machine_type = Some(ShellString::new(raw));
            } else {
                properties.push((ShellString::new(key), ShellString::new(raw)));
            }
        }
        let machine_type = machine_type.filter(|name| !name.as_ref().is_empty()).ok_or_else(|| "Hexagon machine type is required".to_string())?;
        Ok(Self { machine_type, properties })
    }
}
