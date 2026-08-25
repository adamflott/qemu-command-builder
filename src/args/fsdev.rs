use crate::parsers::{ARG_FSDEV, DELIM_COMMA};
use crate::to_command::{ToArg, ToCommand};
use bon::Builder;
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

/// QEMU `security_model=` values for `-fsdev local`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum SecurityModel {
    Passthrough,
    MappedXAttr,
    MappedFile,
    None,
}

impl ToArg for SecurityModel {
    fn to_arg(&self) -> &str {
        match self {
            SecurityModel::Passthrough => "passthrough",
            SecurityModel::MappedXAttr => "mapped-xattr",
            SecurityModel::MappedFile => "mapped-file",
            SecurityModel::None => "none",
        }
    }
}

impl FromStr for SecurityModel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "passthrough" => Ok(Self::Passthrough),
            "mapped-xattr" => Ok(Self::MappedXAttr),
            "mapped-file" => Ok(Self::MappedFile),
            "none" => Ok(Self::None),
            _ => Err(format!("invalid security_model value: {s}")),
        }
    }
}

/// A `-fsdev local,...` backend.
///
/// This exports a host path for a guest 9p device.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct FsDevLocal {
    /// Specifies identifier for this device.
    id: String,

    /// Specifies the export path for the file system device.
    path: PathBuf,

    /// Specifies the security model to be used for this export path.
    security_model: SecurityModel,

    /// Emit `writeout=immediate` when enabled.
    writeout: Option<()>,

    /// Emit `readonly=on` when enabled.
    readonly: Option<()>,

    /// Specifies the default mode for newly created files on the host.
    fmode: Option<String>,

    /// Specifies the default mode for newly created directories on the host.
    dmode: Option<String>,

    /// Maximum number of concurrent extended-attribute FIDs (zero is unlimited).
    max_xattr: Option<usize>,

    /// Throttling limits in bytes per second.
    throttling_bps_total: Option<usize>,
    throttling_bps_read: Option<usize>,
    throttling_bps_write: Option<usize>,

    /// Bursts in bytes per second.
    throttling_bps_total_max: Option<usize>,
    bps_read_max: Option<usize>,
    bps_write_max: Option<usize>,

    /// Request rate limits in requests per second.
    throttling_iops_total: Option<usize>,
    throttling_iops_read: Option<usize>,
    throttling_iops_write: Option<usize>,

    /// Bursts in requests per second.
    throttling_iops_total_max: Option<usize>,
    throttling_iops_read_max: Option<usize>,
    throttling_iops_write_max: Option<usize>,

    /// Request size for IOPS throttling accounting.
    throttling_iops_size: Option<usize>,
}

/// A synthetic `-fsdev synth,...` backend used by QTests.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct FsDevSynth {
    /// Specifies identifier for this device.
    id: String,
    /// Emit `readonly=on` when enabled.
    readonly: Option<()>,
    /// Maximum number of concurrent extended-attribute FIDs (zero is unlimited).
    max_xattr: Option<usize>,
}

/// Define a new QEMU file system device.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum FsDev {
    Local(Box<FsDevLocal>),
    Synth(FsDevSynth),
}

impl ToCommand for FsDev {
    fn command(&self) -> String {
        ARG_FSDEV.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];
        match self {
            FsDev::Local(local) => {
                args.push("local".to_string());
                args.push(format!("id={}", local.id));
                args.push(format!("path={}", local.path.display()));
                args.push(format!("security_model={}", local.security_model.to_arg()));

                if local.writeout.is_some() {
                    args.push("writeout=immediate".to_string());
                }
                if local.readonly.is_some() {
                    args.push("readonly=on".to_string());
                }
                if let Some(fmode) = &local.fmode {
                    args.push(format!("fmode={}", fmode));
                }
                if let Some(dmode) = &local.dmode {
                    args.push(format!("dmode={}", dmode));
                }
                if let Some(max_xattr) = local.max_xattr {
                    args.push(format!("max_xattr={}", max_xattr));
                }
                if let Some(v) = local.throttling_bps_total {
                    args.push(format!("throttling.bps-total={}", v));
                }
                if let Some(v) = local.throttling_bps_read {
                    args.push(format!("throttling.bps-read={}", v));
                }
                if let Some(v) = local.throttling_bps_write {
                    args.push(format!("throttling.bps-write={}", v));
                }
                if let Some(v) = local.throttling_bps_total_max {
                    args.push(format!("throttling.bps-total-max={}", v));
                }
                if let Some(v) = local.bps_read_max {
                    args.push(format!("throttling.bps-read-max={}", v));
                }
                if let Some(v) = local.bps_write_max {
                    args.push(format!("throttling.bps-write-max={}", v));
                }
                if let Some(v) = local.throttling_iops_total {
                    args.push(format!("throttling.iops-total={}", v));
                }
                if let Some(v) = local.throttling_iops_read {
                    args.push(format!("throttling.iops-read={}", v));
                }
                if let Some(v) = local.throttling_iops_write {
                    args.push(format!("throttling.iops-write={}", v));
                }
                if let Some(v) = local.throttling_iops_total_max {
                    args.push(format!("throttling.iops-total-max={}", v));
                }
                if let Some(v) = local.throttling_iops_read_max {
                    args.push(format!("throttling.iops-read-max={}", v));
                }
                if let Some(v) = local.throttling_iops_write_max {
                    args.push(format!("throttling.iops-write-max={}", v));
                }
                if let Some(v) = local.throttling_iops_size {
                    args.push(format!("throttling.iops-size={}", v));
                }
            }
            FsDev::Synth(synth) => {
                args.push("synth".to_string());
                args.push(format!("id={}", synth.id));
                if synth.readonly.is_some() {
                    args.push("readonly=on".to_string());
                }
                if let Some(max_xattr) = synth.max_xattr {
                    args.push(format!("max_xattr={}", max_xattr));
                }
            }
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for FsDev {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(DELIM_COMMA);
        let backend = parts.next().ok_or_else(|| "empty fsdev argument".to_string())?;
        match backend {
            "local" => parse_local_fsdev(parts.collect()),
            "synth" => parse_synth_fsdev(parts.collect()),
            other => Err(format!("unsupported fsdev backend: {other}")),
        }
    }
}

fn parse_local_fsdev(parts: Vec<&str>) -> Result<FsDev, String> {
    let mut id = None;
    let mut path = None;
    let mut security_model = None;
    let mut writeout = None;
    let mut readonly = None;
    let mut fmode = None;
    let mut dmode = None;
    let mut max_xattr = None;
    let mut throttling_bps_total = None;
    let mut throttling_bps_read = None;
    let mut throttling_bps_write = None;
    let mut throttling_bps_total_max = None;
    let mut bps_read_max = None;
    let mut bps_write_max = None;
    let mut throttling_iops_total = None;
    let mut throttling_iops_read = None;
    let mut throttling_iops_write = None;
    let mut throttling_iops_total_max = None;
    let mut throttling_iops_read_max = None;
    let mut throttling_iops_write_max = None;
    let mut throttling_iops_size = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid fsdev local option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "path" => path = Some(PathBuf::from(value)),
            "security_model" => security_model = Some(value.parse::<SecurityModel>()?),
            "writeout" => {
                if value != "immediate" {
                    return Err(format!("invalid writeout value: {value}"));
                }
                writeout = Some(());
            }
            "readonly" => {
                if value != "on" {
                    return Err(format!("invalid readonly value: {value}"));
                }
                readonly = Some(());
            }
            "fmode" => fmode = Some(value.to_string()),
            "dmode" => dmode = Some(value.to_string()),
            "max_xattr" => max_xattr = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "throttling.bps-total" => throttling_bps_total = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "throttling.bps-read" => throttling_bps_read = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "throttling.bps-write" => throttling_bps_write = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "throttling.bps-total-max" => throttling_bps_total_max = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "throttling.bps-read-max" => bps_read_max = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "throttling.bps-write-max" => bps_write_max = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "throttling.iops-total" => throttling_iops_total = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "throttling.iops-read" => throttling_iops_read = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "throttling.iops-write" => throttling_iops_write = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "throttling.iops-total-max" => throttling_iops_total_max = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "throttling.iops-read-max" => throttling_iops_read_max = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "throttling.iops-write-max" => throttling_iops_write_max = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            "throttling.iops-size" => throttling_iops_size = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            other => return Err(format!("unsupported fsdev local option: {other}")),
        }
    }

    Ok(FsDev::Local(Box::new(FsDevLocal {
        id: id.ok_or_else(|| "fsdev local requires id=".to_string())?,
        path: path.ok_or_else(|| "fsdev local requires path=".to_string())?,
        security_model: security_model.ok_or_else(|| "fsdev local requires security_model=".to_string())?,
        writeout,
        readonly,
        fmode,
        dmode,
        max_xattr,
        throttling_bps_total,
        throttling_bps_read,
        throttling_bps_write,
        throttling_bps_total_max,
        bps_read_max,
        bps_write_max,
        throttling_iops_total,
        throttling_iops_read,
        throttling_iops_write,
        throttling_iops_total_max,
        throttling_iops_read_max,
        throttling_iops_write_max,
        throttling_iops_size,
    })))
}

fn parse_synth_fsdev(parts: Vec<&str>) -> Result<FsDev, String> {
    let mut id = None;
    let mut readonly = None;
    let mut max_xattr = None;

    for part in parts {
        let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid fsdev synth option: {part}"))?;
        match key {
            "id" => id = Some(value.to_string()),
            "readonly" => {
                if value != "on" {
                    return Err(format!("invalid readonly value: {value}"));
                }
                readonly = Some(());
            }
            "max_xattr" => max_xattr = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
            other => return Err(format!("unsupported fsdev synth option: {other}")),
        }
    }

    Ok(FsDev::Synth(FsDevSynth {
        id: id.ok_or_else(|| "fsdev synth requires id=".to_string())?,
        readonly,
        max_xattr,
    }))
}
