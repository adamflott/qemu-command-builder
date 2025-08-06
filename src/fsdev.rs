use crate::to_command::ToArg;
use crate::to_command::ToCommand;
use bon::Builder;
use std::path::PathBuf;

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
/// Accesses to the filesystem are done by QEMU
#[derive(Builder)]
pub struct FsDevLocal {
    /// Specifies identifier for this device.
    id: String,

    /// Specifies the export path for the file system device. Files
    /// under this path will be available to the 9p client on the guest.
    path: PathBuf,

    /// Specifies the security model to be used for this export path.
    /// Supported security models are "passthrough", "mapped-xattr",
    /// "mapped-file" and "none". In "passthrough" security model, files
    /// are stored using the same credentials as they are created on the
    /// guest. This requires QEMU to run as root. In "mapped-xattr"
    /// security model, some of the file attributes like uid, gid, mode
    /// bits and link target are stored as file attributes. For
    /// "mapped-file" these attributes are stored in the hidden
    /// .virtfs\_metadata directory. Directories exported by this
    /// security model cannot interact with other unix tools. "none"
    /// security model is same as passthrough except the sever won't
    /// report failures if it fails to set file attributes like
    /// ownership. Security model is mandatory only for local fsdriver.
    security_model: SecurityModel,

    /// This is an optional argument. The only supported value is
    /// "immediate". This means that host page cache will be used to
    /// read and write data but write notification will be sent to the
    /// guest only when the data has been reported as written by the
    /// storage subsystem.
    writeout: Option<()>,

    /// Enables exporting 9p share as a readonly mount for guests. By
    /// default read-write access is given.
    readonly: Option<()>,

    /// Specifies the default mode for newly created files on the host.
    /// Works only with security models "mapped-xattr" and
    /// "mapped-file".
    fmode: Option<String>,

    /// Specifies the default mode for newly created directories on the
    /// host. Works only with security models "mapped-xattr" and
    /// "mapped-file".
    dmode: Option<String>,

    /// Specify bandwidth throttling limits in bytes per second, either
    /// for all request types or for reads or writes only.
    throttling_bps_total: Option<usize>,
    throttling_bps_read: Option<usize>,
    throttling_bps_write: Option<usize>,

    /// Specify bursts in bytes per second, either for all request types
    /// or for reads or writes only. Bursts allow the guest I/O to spike
    /// above the limit temporarily.
    throttling_bps_total_max: Option<usize>,
    bps_read_max: Option<usize>,
    bps_write_max: Option<usize>,

    /// Specify request rate limits in requests per second, either for
    /// all request types or for reads or writes only.
    throttling_iops_total: Option<usize>,
    throttling_iops_read: Option<usize>,
    throttling_iops_write: Option<usize>,

    /// Specify bursts in requests per second, either for all request
    /// types or for reads or writes only. Bursts allow the guest I/O to
    /// spike above the limit temporarily.
    throttling_iops_total_max: Option<usize>,
    throttling_iops_read_max: Option<usize>,
    throttling_iops_write_max: Option<usize>,

    /// Let every is bytes of a request count as a new request for iops
    /// throttling purposes.
    throttling_iops_size: Option<usize>,
}

/// Synthetic filesystem, only used by QTests.
pub struct FsDevSynth {
    /// Specifies identifier for this device.
    id: String,
}

/// Define a new file system device
///
/// TODO
/// - device virtio-9p-type integration
pub enum FsDev {
    Local(FsDevLocal),
    Synth(FsDevSynth),
}

impl ToCommand for FsDev {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-fsdev".to_string());

        match self {
            FsDev::Local(local) => {
                let mut arg = vec![];

                arg.push(format!("local,id={}", local.id));

                arg.push(format!(",path={}", local.path.display()));

                arg.push(format!(",security-model={}", local.security_model.to_arg()));

                if local.writeout.is_some() {
                    arg.push("writeout=immediate".to_string());
                }

                if local.readonly.is_some() {
                    arg.push("readonly=on".to_string());
                }

                if let Some(fmode) = &local.fmode {
                    arg.push(format!("fmode={}", fmode));
                }

                if let Some(dmode) = &local.dmode {
                    arg.push(format!("dmode={}", dmode));
                }

                if let Some(throttling_bps_total) = local.throttling_bps_total {
                    arg.push(format!("throttling.bps-total={}", throttling_bps_total));
                }
                if let Some(throttling_bps_read) = local.throttling_bps_read {
                    arg.push(format!("throttling.bps-read={}", throttling_bps_read));
                }
                if let Some(throttling_bps_write) = local.throttling_bps_write {
                    arg.push(format!("throttling.bps-write={}", throttling_bps_write));
                }

                if let Some(throttling_bps_total_max) = local.throttling_bps_total_max {
                    arg.push(format!(
                        "throttling.bps-total-max={}",
                        throttling_bps_total_max
                    ));
                }
                if let Some(bps_read_max) = local.bps_read_max {
                    arg.push(format!("bps-read-max={}", bps_read_max));
                }
                if let Some(bps_write_max) = local.bps_write_max {
                    arg.push(format!("bps-write-max={}", bps_write_max));
                }

                if let Some(throttling_iops_total) = local.throttling_iops_total {
                    arg.push(format!("throttling.iops-total={}", throttling_iops_total));
                }
                if let Some(throttling_iops_read) = local.throttling_iops_read {
                    arg.push(format!("throttling.iops-read={}", throttling_iops_read));
                }
                if let Some(throttling_iops_write) = local.throttling_iops_write {
                    arg.push(format!("throttling.iops-write={}", throttling_iops_write));
                }

                if let Some(throttling_ios_total_max) = local.throttling_iops_total_max {
                    arg.push(format!(
                        "throttling.ios-total-max={}",
                        throttling_ios_total_max
                    ));
                }
                if let Some(throttling_iops_read_max) = local.throttling_iops_read_max {
                    arg.push(format!(
                        "throttling.iops-read-max={}",
                        throttling_iops_read_max
                    ));
                }
                if let Some(throttling_iops_write_max) = local.throttling_iops_write_max {
                    arg.push(format!(
                        "throttling.iops-write-max={}",
                        throttling_iops_write_max
                    ));
                }

                if let Some(throttling_iops_size) = local.throttling_iops_size {
                    arg.push(format!("throttling.iops-size={}", throttling_iops_size));
                }
                cmd.push(arg.join(","));
            }
            FsDev::Synth(synth) => {
                let mut arg = String::new();
                arg.push_str("synth,id=");
                arg.push_str(synth.id.to_string().as_str());
                cmd.push(arg);
            }
        }

        cmd
    }
}
