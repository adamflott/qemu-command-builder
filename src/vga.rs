use crate::parsers::ARG_VGA;
use crate::to_command::ToCommand;
use proptest_derive::Arbitrary;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum VGA {
    /// Cirrus Logic GD5446 Video card. All Windows versions starting
    /// from Windows 95 should recognize and use this graphic card. For
    /// optimal performances, use 16 bit color depth in the guest and
    /// the host OS. (This card was the default before QEMU 2.2)
    Cirrus,

    /// Standard VGA card with Bochs VBE extensions. If your guest OS
    /// supports the VESA 2.0 VBE extensions (e.g. Windows XP) and if
    /// you want to use high resolution modes (>= 1280x1024x16) then you
    /// should use this option. (This card is the default since QEMU
    /// 2.2)
    Std,

    /// VMWare SVGA-II compatible adapter. Use it if you have
    /// sufficiently recent XFree86/XOrg server or Windows guest with a
    /// driver for this card.
    Vmware,

    /// QXL paravirtual graphic card. It is VGA compatible (including
    /// VESA 2.0 VBE support). Works best with qxl guest drivers
    /// installed though. Recommended choice when using the spice
    /// protocol.
    Qxl,

    /// (sun4m only) Sun TCX framebuffer. This is the default
    /// framebuffer for sun4m machines and offers both 8-bit and 24-bit
    /// colour depths at a fixed resolution of 1024x768.
    Tcx,

    /// (sun4m only) Sun cgthree framebuffer. This is a simple 8-bit
    /// framebuffer for sun4m machines available in both 1024x768
    /// (OpenBIOS) and 1152x900 (OBP) resolutions aimed at people
    /// wishing to run older Solaris versions.
    Cg3,

    /// Virtio VGA card.
    Virtio,

    /// Disable VGA card.
    None,
}

impl FromStr for VGA {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cirrus" => Ok(VGA::Cirrus),
            "std" => Ok(VGA::Std),
            "vmware" => Ok(VGA::Vmware),
            "qxl" => Ok(VGA::Qxl),
            "tcx" => Ok(VGA::Tcx),
            "cg3" => Ok(VGA::Cg3),
            "virtio" => Ok(VGA::Virtio),
            "none" => Ok(VGA::None),
            other => Err(format!("Unknown vga type: {}", other)),
        }
    }
}

impl Display for VGA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VGA::Cirrus => write!(f, "cirrus"),
            VGA::Std => write!(f, "std"),
            VGA::Vmware => write!(f, "vmware"),
            VGA::Qxl => write!(f, "qxl"),
            VGA::Tcx => write!(f, "tcx"),
            VGA::Cg3 => write!(f, "cg3"),
            VGA::Virtio => write!(f, "virtio"),
            VGA::None => write!(f, "none"),
        }
    }
}

impl ToCommand for VGA {
    fn command(&self) -> String {
        ARG_VGA.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];

        match self {
            VGA::Cirrus => {
                args.push("cirrus".to_string());
            }
            VGA::Std => {
                args.push("std".to_string());
            }
            VGA::Vmware => {
                args.push("vmware".to_string());
            }
            VGA::Qxl => {
                args.push("qxl".to_string());
            }
            VGA::Tcx => {
                args.push("tcx".to_string());
            }
            VGA::Cg3 => {
                args.push("cg3".to_string());
            }
            VGA::Virtio => {
                args.push("virtio".to_string());
            }
            VGA::None => {
                args.push("none".to_string());
            }
        }
        args
    }
}
