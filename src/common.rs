use std::fmt::{Display, Formatter};
use std::str::FromStr;

use proptest_derive::Arbitrary;

use crate::to_command::{ToArg, ToCommand};

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
/// A QEMU boolean-like value encoded as `"yes"` or `"no"`.
pub enum YesNo {
    /// Emits `"yes"`.
    Yes,
    /// Emits `"no"`.
    No,
}

impl Display for YesNo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            YesNo::Yes => write!(f, "yes"),
            YesNo::No => write!(f, "no"),
        }
    }
}
impl FromStr for YesNo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yes" => Ok(YesNo::Yes),
            "no" => Ok(YesNo::No),
            _ => Err(()),
        }
    }
}

impl ToCommand for YesNo {
    fn to_args(&self) -> Vec<String> {
        let mut cmd = vec![];
        match self {
            YesNo::Yes => {
                cmd.push("yes".to_string());
            }
            YesNo::No => {
                cmd.push("no".to_string());
            }
        }
        cmd
    }
}

impl ToArg for YesNo {
    fn to_arg(&self) -> &str {
        match self {
            YesNo::Yes => "yes",
            YesNo::No => "no",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
/// A QEMU boolean-like value encoded as `"on"` or `"off"`.
pub enum OnOff {
    /// Emits `"on"`.
    On,
    /// Emits `"off"`.
    Off,
}

impl Display for OnOff {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OnOff::On => write!(f, "on"),
            OnOff::Off => write!(f, "off"),
        }
    }
}

impl FromStr for OnOff {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(OnOff::On),
            "off" => Ok(OnOff::Off),
            _ => Err(()),
        }
    }
}

impl ToCommand for OnOff {
    fn to_args(&self) -> Vec<String> {
        let mut cmd = vec![];
        match self {
            OnOff::On => {
                cmd.push("on".to_string());
            }
            OnOff::Off => {
                cmd.push("off".to_string());
            }
        }
        cmd
    }
}

impl ToArg for OnOff {
    fn to_arg(&self) -> &str {
        match self {
            OnOff::On => "on",
            OnOff::Off => "off",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Arbitrary)]
/// A QEMU tri-state value encoded as `"on"`, `"off"`, or `"auto"`.
///
/// The default is [`OnOffAuto::Auto`].
pub enum OnOffAuto {
    /// Emits `"on"`.
    On,
    /// Emits `"off"`.
    Off,
    /// Emits `"auto"`.
    #[default]
    Auto,
}

impl FromStr for OnOffAuto {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(OnOffAuto::On),
            "off" => Ok(OnOffAuto::Off),
            "auto" => Ok(OnOffAuto::Auto),
            _ => Err(()),
        }
    }
}

impl Display for OnOffAuto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OnOffAuto::On => write!(f, "on"),
            OnOffAuto::Off => write!(f, "off"),
            OnOffAuto::Auto => write!(f, "auto"),
        }
    }
}
impl ToCommand for OnOffAuto {
    fn to_args(&self) -> Vec<String> {
        let mut cmd = vec![];
        match self {
            OnOffAuto::On => {
                cmd.push("on".to_string());
            }
            OnOffAuto::Off => {
                cmd.push("off".to_string());
            }
            OnOffAuto::Auto => {
                cmd.push("auto".to_string());
            }
        }
        cmd
    }
}

impl ToArg for OnOffAuto {
    fn to_arg(&self) -> &str {
        match self {
            OnOffAuto::On => "on",
            OnOffAuto::Off => "off",
            OnOffAuto::Auto => "auto",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Arbitrary)]
/// A QEMU on/off value whose Rust default is `"on"`.
///
/// The default is [`OnOffDefaultOn::On`].
pub enum OnOffDefaultOn {
    /// Emits `"on"`.
    #[default]
    On,
    /// Emits `"off"`.
    Off,
}

impl FromStr for OnOffDefaultOn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(OnOffDefaultOn::On),
            "off" => Ok(OnOffDefaultOn::Off),
            _ => Err(()),
        }
    }
}

impl Display for OnOffDefaultOn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OnOffDefaultOn::On => write!(f, "on"),
            OnOffDefaultOn::Off => write!(f, "off"),
        }
    }
}
impl ToCommand for OnOffDefaultOn {
    fn to_args(&self) -> Vec<String> {
        let mut cmd = vec![];
        match self {
            OnOffDefaultOn::On => {
                cmd.push("on".to_string());
            }
            OnOffDefaultOn::Off => {
                cmd.push("off".to_string());
            }
        }
        cmd
    }
}

impl ToArg for OnOffDefaultOn {
    fn to_arg(&self) -> &str {
        match self {
            OnOffDefaultOn::On => "on",
            OnOffDefaultOn::Off => "off",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Arbitrary)]
/// A QEMU on/off value whose Rust default is `"off"`.
///
/// The default is [`OnOffDefaultOff::Off`].
pub enum OnOffDefaultOff {
    /// Emits `"on"`.
    On,
    /// Emits `"off"`.
    #[default]
    Off,
}

impl Display for OnOffDefaultOff {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OnOffDefaultOff::On => write!(f, "on"),
            OnOffDefaultOff::Off => write!(f, "off"),
        }
    }
}

impl FromStr for OnOffDefaultOff {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(OnOffDefaultOff::On),
            "off" => Ok(OnOffDefaultOff::Off),
            _ => Err(()),
        }
    }
}

impl ToCommand for OnOffDefaultOff {
    fn to_args(&self) -> Vec<String> {
        let mut cmd = vec![];
        match self {
            OnOffDefaultOff::On => {
                cmd.push("on".to_string());
            }
            OnOffDefaultOff::Off => {
                cmd.push("off".to_string());
            }
        }
        cmd
    }
}

impl ToArg for OnOffDefaultOff {
    fn to_arg(&self) -> &str {
        match self {
            OnOffDefaultOff::On => "on",
            OnOffDefaultOff::Off => "off",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Arbitrary)]
/// Supported QEMU accelerator backends.
///
/// The default is [`AccelType::Tcg`].
pub enum AccelType {
    /// Kernel-based Virtual Machine acceleration.
    Kvm,
    /// Xen acceleration.
    Xen,
    /// Hypervisor.framework acceleration.
    Hvf,
    /// NetBSD NVMM acceleration.
    Nvmm,
    /// Windows Hypervisor Platform acceleration.
    Whpx,
    /// Tiny Code Generator emulation.
    #[default]
    Tcg,
}

impl ToArg for AccelType {
    fn to_arg(&self) -> &str {
        match self {
            AccelType::Kvm => "kvm",
            AccelType::Xen => "xen",
            AccelType::Hvf => "hvf",
            AccelType::Nvmm => "nvmm",
            AccelType::Whpx => "whpx",
            AccelType::Tcg => "tcg",
        }
    }
}

impl FromStr for AccelType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kvm" => Ok(AccelType::Kvm),
            "xen" => Ok(AccelType::Xen),
            "hvf" => Ok(AccelType::Hvf),
            "nvmm" => Ok(AccelType::Nvmm),
            "whpx" => Ok(AccelType::Whpx),
            "tcg" => Ok(AccelType::Tcg),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
/// A QEMU setting encoded as `"ignore"` or `"unmap"`.
pub enum IgnoreUnmap {
    /// Emits `"ignore"`.
    Ignore,
    /// Emits `"unmap"`.
    Unmap,
}

impl Display for IgnoreUnmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IgnoreUnmap::Ignore => write!(f, "ignore"),
            IgnoreUnmap::Unmap => write!(f, "unmap"),
        }
    }
}
impl FromStr for IgnoreUnmap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ignore" => Ok(IgnoreUnmap::Ignore),
            "unmap" => Ok(IgnoreUnmap::Unmap),
            _ => Err(()),
        }
    }
}
impl ToArg for IgnoreUnmap {
    fn to_arg(&self) -> &str {
        match self {
            IgnoreUnmap::Ignore => "ignore",
            IgnoreUnmap::Unmap => "unmap",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
/// A QEMU value encoded as `"on"`, `"off"`, or `"unmap"`.
pub enum OnOffUnmap {
    /// Emits `"on"`.
    On,
    /// Emits `"off"`.
    Off,
    /// Emits `"unmap"`.
    Unmap,
}

impl Display for OnOffUnmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OnOffUnmap::On => write!(f, "on"),
            OnOffUnmap::Off => write!(f, "off"),
            OnOffUnmap::Unmap => write!(f, "unmap"),
        }
    }
}

impl FromStr for OnOffUnmap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(OnOffUnmap::On),
            "off" => Ok(OnOffUnmap::Off),
            "unmap" => Ok(OnOffUnmap::Unmap),
            _ => Err(()),
        }
    }
}
impl ToArg for OnOffUnmap {
    fn to_arg(&self) -> &str {
        match self {
            OnOffUnmap::On => "on",
            OnOffUnmap::Off => "off",
            OnOffUnmap::Unmap => "unmap",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
/// A QEMU policy value encoded as `"auto"`, `"never"`, or `"always"`.
pub enum AutoNeverAlways {
    /// Emits `"auto"`.
    Auto,
    /// Emits `"never"`.
    Never,
    /// Emits `"always"`.
    Always,
}

impl ToArg for AutoNeverAlways {
    fn to_arg(&self) -> &str {
        match self {
            AutoNeverAlways::Auto => "auto",
            AutoNeverAlways::Never => "never",
            AutoNeverAlways::Always => "always",
        }
    }
}
