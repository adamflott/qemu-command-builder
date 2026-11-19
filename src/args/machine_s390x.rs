use crate::common::{AccelType, OnOffDefaultOff};
use crate::parsers::ARG_MACHINE;
use crate::shell_string::ShellString;
use crate::to_command::{ToArg, ToCommand};
use bon::Builder;
use proptest_derive::Arbitrary;
use std::collections::BTreeMap;
use std::str::FromStr;

/// An s390x `-machine` definition, including QEMU 11.2 secure IPL options.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct MachineS390x {
    /// Machine name, such as `s390-ccw-virtio` or `s390-ccw-virtio-11.2`.
    #[builder(into)]
    machine_type: ShellString,
    accel: Option<Vec<AccelType>>,
    /// Paths to X.509 certificate files or directories containing certificates.
    /// Requires a machine version supporting secure IPL (QEMU 11.2 or newer).
    boot_certs: Option<Vec<ShellString>>,
    /// Enable secure boot on s390-ccw guests. QEMU's default is off.
    secure_boot: Option<OnOffDefaultOff>,
    /// Additional board properties, such as `aes-key-wrap` or `memory-backend`.
    extra_properties: Option<Vec<(ShellString, ShellString)>>,
}

impl ToCommand for MachineS390x {
    fn command(&self) -> String {
        ARG_MACHINE.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut args = vec![self.machine_type.as_ref().to_string()];
        if let Some(accels) = &self.accel {
            args.push(format!("accel={}", accels.iter().map(ToArg::to_arg).collect::<Vec<_>>().join(":")));
        }
        if let Some(certs) = &self.boot_certs {
            for (index, path) in certs.iter().enumerate() {
                args.push(format!("boot-certs.{index}.path={}", path.as_ref()));
            }
        }
        if let Some(secure_boot) = &self.secure_boot {
            args.push(format!("secure-boot={}", secure_boot.to_arg()));
        }
        if let Some(properties) = &self.extra_properties {
            for (key, value) in properties {
                args.push(format!("{}={}", key.as_ref(), value.as_ref()));
            }
        }
        vec![args.join(",")]
    }
}

impl FromStr for MachineS390x {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut machine_type = None;
        let mut accel = None;
        let mut boot_certs = BTreeMap::new();
        let mut secure_boot = None;
        let mut extra_properties = Vec::new();
        for (position, part) in value.split(',').enumerate() {
            if position == 0 && !part.contains('=') {
                machine_type = Some(ShellString::new(part));
                continue;
            }
            let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid s390x machine option: {part}"))?;
            match key {
                "type" => machine_type = Some(ShellString::new(raw)),
                "accel" => {
                    accel = Some(
                        raw.split(':')
                            .map(|v| v.parse::<AccelType>().map_err(|_| format!("invalid accel value: {v}")))
                            .collect::<Result<Vec<_>, _>>()?,
                    );
                }
                "secure-boot" => {
                    secure_boot = Some(raw.parse::<OnOffDefaultOff>().map_err(|_| format!("invalid secure-boot value: {raw}"))?);
                }
                _ if key.starts_with("boot-certs.") => {
                    let index = key
                        .strip_prefix("boot-certs.")
                        .and_then(|key| key.strip_suffix(".path"))
                        .ok_or_else(|| format!("invalid boot certificate option: {key}"))?;
                    let index = index.parse::<usize>().map_err(|_| format!("invalid boot certificate index: {index}"))?;
                    boot_certs.insert(index, ShellString::new(raw));
                }
                _ => extra_properties.push((ShellString::new(key), ShellString::new(raw))),
            }
        }
        let machine_type = machine_type.filter(|name| !name.as_ref().is_empty()).ok_or_else(|| "s390x machine type is required".to_string())?;
        // QEMU keyval lists must start at zero and have no gaps.
        for (expected, index) in boot_certs.keys().enumerate() {
            if expected != *index {
                return Err(format!("boot-certs indices must be contiguous from zero: expected {expected}, got {index}"));
            }
        }
        Ok(Self {
            machine_type,
            accel,
            boot_certs: (!boot_certs.is_empty()).then(|| boot_certs.into_values().collect()),
            secure_boot,
            extra_properties: (!extra_properties.is_empty()).then_some(extra_properties),
        })
    }
}
