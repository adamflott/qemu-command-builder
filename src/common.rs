use crate::to_command::{ToArg, ToCommand};

pub enum YesNo {
    Yes,
    No,
}

impl ToCommand for YesNo {
    fn to_command(&self) -> Vec<String> {
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

pub enum OnOff {
    On,
    Off,
}

impl ToCommand for OnOff {
    fn to_command(&self) -> Vec<String> {
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

#[derive(Default)]
pub enum OnOffAuto {
    On,
    Off,
    #[default]
    Auto,
}

impl ToCommand for OnOffAuto {
    fn to_command(&self) -> Vec<String> {
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

#[derive(Default)]
pub enum OnOffDefaultOn {
    #[default]
    On,
    Off,
}

impl ToCommand for OnOffDefaultOn {
    fn to_command(&self) -> Vec<String> {
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

#[derive(Default)]
pub enum OnOffDefaultOff {
    On,
    #[default]
    Off,
}
impl ToCommand for OnOffDefaultOff {
    fn to_command(&self) -> Vec<String> {
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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum AccelType {
    Kvm,
    Xen,
    Hvf,
    Nvmm,
    Whpx,
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

pub enum IgnoreUnmap {
    Ignore,
    Unmap,
}

impl ToArg for IgnoreUnmap {
    fn to_arg(&self) -> &str {
        match self {
            IgnoreUnmap::Ignore => "ignore",
            IgnoreUnmap::Unmap => "unmap",
        }
    }
}

pub enum OnOffUnmap {
    On,
    Off,
    Unmap,
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

pub enum AutoNeverAlways {
    Auto,
    Never,
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
