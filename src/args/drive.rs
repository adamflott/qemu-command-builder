use crate::parsers::ARG_DRIVE;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::common::{IgnoreUnmap, OnOff, OnOffUnmap};
use crate::parsers::DELIM_COMMA;
use crate::qao;
use crate::shell_path::ShellPath;
use crate::shell_string::ShellString;
use crate::to_command::{ToArg, ToCommand};

const KEY_FILE: &str = "file=";
const KEY_INTERFACE: &str = "if=";
const KEY_BUS: &str = "bus=";
const KEY_UNIT: &str = "unit=";
const KEY_INDEX: &str = "index=";
const KEY_MEDIA: &str = "media=";
const KEY_SNAPSHOT: &str = "snapshot=";
const KEY_CACHE: &str = "cache=";
const KEY_CACHE_DIRECT: &str = "cache.direct=";
const KEY_ID: &str = "id=";
const KEY_AIO: &str = "aio=";
const KEY_FORMAT: &str = "format=";
const KEY_ENCRYPT_FORMAT: &str = "encrypt.format=";
const KEY_ENCRYPT_KEY_SECRET: &str = "encrypt.key-secret=";
const KEY_RERROR: &str = "rerror=";
const KEY_WERROR: &str = "werror=";
const KEY_COPY_ON_READ: &str = "copy-on-read=";
const KEY_BPS: &str = "bps=";
const KEY_BPS_RD: &str = "bps_rd=";
const KEY_BPS_WR: &str = "bps_wr=";
const KEY_BPS_MAX: &str = "bps_max=";
const KEY_BPS_RD_MAX: &str = "bps_rd_max=";
const KEY_BPS_WR_MAX: &str = "bps_wr_max=";
const KEY_IOPS: &str = "iops=";
const KEY_IOPS_RD: &str = "iops_rd=";
const KEY_IOPS_WR: &str = "iops_wr=";
const KEY_IOPS_MAX: &str = "iops_max=";
const KEY_IOPS_RD_MAX: &str = "iops_rd_max=";
const KEY_IOPS_WR_MAX: &str = "iops_wr_max=";
const KEY_IOPS_SIZE: &str = "iops_size=";
const KEY_GROUP: &str = "group=";
const KEY_THROTTLING_BPS_TOTAL: &str = "throttling.bps-total=";
const KEY_THROTTLING_BPS_TOTAL_MAX: &str = "throttling.bps-total-max=";
const KEY_THROTTLING_BPS_TOTAL_MAX_LENGTH: &str = "throttling.bps-total-max-length=";
const KEY_THROTTLING_IOPS_TOTAL: &str = "throttling.iops-total=";
const KEY_THROTTLING_IOPS_TOTAL_MAX: &str = "throttling.iops-total-max=";
const KEY_THROTTLING_IOPS_TOTAL_MAX_LENGTH: &str = "throttling.iops-total-max-length=";
const KEY_DISCARD: &str = "discard=";
const KEY_READ_ONLY: &str = "readonly=";
const KEY_AUTO_READ_ONLY: &str = "auto-read-only=";
const KEY_DETECT_ZEROES: &str = "detect-zeroes=";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum DriveInterface {
    Ide,
    Scsi,
    Sd,
    Mtd,
    Floppy,
    Pflash,
    Virtio,
    None,
}

impl Display for DriveInterface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DriveInterface::Ide => f.write_str("ide"),
            DriveInterface::Scsi => f.write_str("scsi"),
            DriveInterface::Sd => f.write_str("sd"),
            DriveInterface::Mtd => f.write_str("mtd"),
            DriveInterface::Floppy => f.write_str("floppy"),
            DriveInterface::Pflash => f.write_str("pflash"),
            DriveInterface::Virtio => f.write_str("virtio"),
            DriveInterface::None => f.write_str("none"),
        }
    }
}

impl FromStr for DriveInterface {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ide" => Ok(DriveInterface::Ide),
            "scsi" => Ok(DriveInterface::Scsi),
            "sd" => Ok(DriveInterface::Sd),
            "mtd" => Ok(DriveInterface::Mtd),
            "floppy" => Ok(DriveInterface::Floppy),
            "pflash" => Ok(DriveInterface::Pflash),
            "virtio" => Ok(DriveInterface::Virtio),
            "none" => Ok(DriveInterface::None),
            _ => Err(()),
        }
    }
}
impl ToArg for DriveInterface {
    fn to_arg(&self) -> &str {
        match self {
            DriveInterface::Ide => "ide",
            DriveInterface::Scsi => "scsi",
            DriveInterface::Sd => "sd",
            DriveInterface::Mtd => "mtd",
            DriveInterface::Floppy => "floppy",
            DriveInterface::Pflash => "pflash",
            DriveInterface::Virtio => "virtio",
            DriveInterface::None => "none",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum DriveMedia {
    Disk,
    Cdrom,
}

impl Display for DriveMedia {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DriveMedia::Disk => f.write_str("disk"),
            DriveMedia::Cdrom => f.write_str("cdrom"),
        }
    }
}
impl FromStr for DriveMedia {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "disk" => Ok(DriveMedia::Disk),
            "cdrom" => Ok(DriveMedia::Cdrom),
            _ => Err(()),
        }
    }
}
impl ToArg for DriveMedia {
    fn to_arg(&self) -> &str {
        match self {
            DriveMedia::Disk => "disk",
            DriveMedia::Cdrom => "cdrom",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Arbitrary)]
pub enum DriveCacheType {
    None,
    #[default]
    Writeback,
    Writethrough,
    Unsafe,
    Directsync,
}

impl Display for DriveCacheType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DriveCacheType::None => f.write_str("none"),
            DriveCacheType::Writeback => f.write_str("writeback"),
            DriveCacheType::Writethrough => f.write_str("writethrough"),
            DriveCacheType::Unsafe => f.write_str("unsafe"),
            DriveCacheType::Directsync => f.write_str("directsync"),
        }
    }
}
impl FromStr for DriveCacheType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(DriveCacheType::None),
            "writeback" => Ok(DriveCacheType::Writeback),
            "writethrough" => Ok(DriveCacheType::Writethrough),
            "unsafe" => Ok(DriveCacheType::Unsafe),
            "directsync" => Ok(DriveCacheType::Directsync),
            _ => Err(()),
        }
    }
}
impl ToArg for DriveCacheType {
    fn to_arg(&self) -> &str {
        match self {
            DriveCacheType::None => "none",
            DriveCacheType::Writeback => "writeback",
            DriveCacheType::Writethrough => "writethrough",
            DriveCacheType::Unsafe => "unsafe",
            DriveCacheType::Directsync => "directsync",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum DriveAIOType {
    Threads,
    Native,
    IoUring,
}

impl Display for DriveAIOType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DriveAIOType::Threads => write!(f, "threads"),
            DriveAIOType::Native => write!(f, "native"),
            DriveAIOType::IoUring => write!(f, "io_uring"),
        }
    }
}
impl FromStr for DriveAIOType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "threads" => Ok(DriveAIOType::Threads),
            "native" => Ok(DriveAIOType::Native),
            "io_uring" => Ok(DriveAIOType::IoUring),
            _ => Err(()),
        }
    }
}
impl ToArg for DriveAIOType {
    fn to_arg(&self) -> &str {
        match self {
            DriveAIOType::Threads => "threads",
            DriveAIOType::Native => "native",
            DriveAIOType::IoUring => "io_uring",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum DriveErrorAction {
    Ignore,
    Stop,
    Report,
    Enospc,
}

impl Display for DriveErrorAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DriveErrorAction::Ignore => write!(f, "ignore"),
            DriveErrorAction::Stop => write!(f, "stop"),
            DriveErrorAction::Report => write!(f, "report"),
            DriveErrorAction::Enospc => write!(f, "enospc"),
        }
    }
}
impl FromStr for DriveErrorAction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ignore" => Ok(DriveErrorAction::Ignore),
            "stop" => Ok(DriveErrorAction::Stop),
            "report" => Ok(DriveErrorAction::Report),
            "enospc" => Ok(DriveErrorAction::Enospc),
            _ => Err(()),
        }
    }
}
impl ToArg for DriveErrorAction {
    fn to_arg(&self) -> &str {
        match self {
            DriveErrorAction::Ignore => "ignore",
            DriveErrorAction::Stop => "stop",
            DriveErrorAction::Report => "report",
            DriveErrorAction::Enospc => "enospc",
        }
    }
}

/// Define a new drive. This includes creating a block driver node (the
/// backend) as well as a guest device, and is mostly a shortcut for
/// defining the corresponding ``-blockdev`` and ``-device`` options.
///
/// ``-drive`` accepts all options that are accepted by ``-blockdev``.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Drive {
    // -drive
    /// This option defines which disk image (see the :ref:`disk images`
    /// chapter in the System Emulation Users Guide) to use with this drive.
    /// If the filename contains comma, you must double it (for instance,
    /// "file=my,,file" to use file "my,file").
    ///
    /// Special files such as iSCSI devices can be specified using
    /// protocol specific URLs. See the section for "Device URL Syntax"
    /// for more information.
    pub file: Option<ShellPath>,

    /// This option defines on which type on interface the drive is
    /// connected. Available types are: ide, scsi, sd, mtd, floppy,
    /// pflash, virtio, none.
    pub interface: Option<DriveInterface>,

    /// These options define where is connected the drive by defining
    /// the bus number and the unit id.
    pub bus: Option<u64>,
    pub unit: Option<u64>,

    /// This option defines where the drive is connected by using an
    /// index in the list of available connectors of a given interface
    /// type.
    pub index: Option<ShellString>,

    /// This option defines the type of the media: disk or cdrom.
    pub media: Option<DriveMedia>,

    /// snapshot is "on" or "off" and controls snapshot mode for the
    /// given drive (see ``-snapshot``).
    pub snapshot: Option<OnOff>,

    /// cache is "none", "writeback", "unsafe", "directsync" or
    /// "writethrough" and controls how the host cache is used to access
    /// block data. This is a shortcut that sets the ``cache.direct``
    /// and ``cache.no-flush`` options (as in ``-blockdev``), and
    /// additionally ``cache.writeback``, which provides a default for
    /// the ``write-cache`` option of block guest devices (as in
    /// ``-device``). The modes correspond to the following settings:
    ///
    /// =============  ===============   ============   ==============
    /// \              cache.writeback   cache.direct   cache.no-flush
    /// =============  ===============   ============   ==============
    /// writeback      on                off            off
    /// none           on                on             off
    /// writethrough   off               off            off
    /// directsync     off               on             off
    /// unsafe         on                off            on
    /// =============  ===============   ============   ==============
    ///
    /// The default mode is ``cache=writeback``.
    pub cache: Option<DriveCacheType>,

    /// Avoid the host page cache when enabled.
    pub cache_direct: Option<OnOff>,

    /// aio is "threads", "native", or "io_uring" and selects between pthread
    /// based disk I/O, native Linux AIO, or Linux io_uring API.
    pub aio: Option<DriveAIOType>,

    /// Specify which disk format will be used rather than detecting the
    /// format. Can be used to specify format=raw to avoid interpreting
    /// an untrusted format header.
    pub format: Option<ShellString>,

    /// Encryption format passed through to the block layer.
    pub encrypt_format: Option<ShellString>,

    /// Secret object id used as the encryption key.
    pub encrypt_key_secret: Option<ShellString>,

    /// Specify which action to take on write and read errors. Valid
    /// actions are: "ignore" (ignore the error and try to continue),
    /// "stop" (pause QEMU), "report" (report the error to the guest),
    /// "enospc" (pause QEMU only if the host disk is full; report the
    /// error to the guest otherwise). The default setting is
    /// ``werror=enospc`` and ``rerror=report``.
    pub rerror: Option<DriveErrorAction>,
    pub werror: Option<DriveErrorAction>,

    pub id: Option<ShellString>,

    /// Open the node read-only. Guest write attempts will fail.
    ///
    /// Note that some block drivers support only read-only access,
    /// either generally or in certain configurations. In this case,
    /// the default value ``readonly=off`` does not work and the
    /// option must be specified explicitly.
    pub read_only: Option<OnOff>,

    /// Allow QEMU to fall back to read-only usage when requested access fails.
    pub auto_read_only: Option<OnOff>,

    /// `copy-on-read=` is `on` or `off` and enables whether to copy read
    /// backing file sectors into the image file.
    pub copy_on_read: Option<OnOff>,

    /// discard is one of "ignore" (or "off") or "unmap" (or "on")
    /// and controls whether ``discard`` (also known as ``trim`` or
    /// ``unmap``) requests are ignored or passed to the filesystem.
    /// Some machine types may not support discard requests.
    pub discard: Option<IgnoreUnmap>,

    /// detect-zeroes is "off", "on" or "unmap" and enables the
    /// automatic conversion of plain zero writes by the OS to
    /// driver specific optimized zero write commands. You may even
    /// choose "unmap" if discard is set to "unmap" to allow a zero
    /// write to be converted to an ``unmap`` operation.
    pub detect_zeroes: Option<OnOffUnmap>,

    /// Specify bandwidth throttling limits in bytes per second, either
    /// for all request types or for reads or writes only. Small values
    /// can lead to timeouts or hangs inside the guest. A safe minimum
    /// for disks is 2 MB/s.
    pub bps: Option<u64>,
    pub bps_rd: Option<u64>,
    pub bps_wr: Option<u64>,

    /// Specify bursts in bytes per second, either for all request types
    /// or for reads or writes only. Bursts allow the guest I/O to spike
    /// above the limit temporarily.
    pub bps_max: Option<u64>,
    pub bps_rd_max: Option<u64>,
    pub bps_wr_max: Option<u64>,

    /// Specify request rate limits in requests per second, either for
    /// all request types or for reads or writes only.
    pub iops: Option<u64>,
    pub iops_rd: Option<u64>,
    pub iops_wr: Option<u64>,

    /// Specify bursts in requests per second, either for all request
    /// types or for reads or writes only. Bursts allow the guest I/O to
    /// spike above the limit temporarily.
    pub iops_max: Option<u64>,
    pub iops_rd_max: Option<u64>,
    pub iops_wr_max: Option<u64>,

    /// Let every is bytes of a request count as a new request for iops
    /// throttling purposes. Use this option to prevent guests from
    /// circumventing iops limits by sending fewer but larger requests.
    pub iops_size: Option<u64>,

    /// Join a throttling quota group with given name g. All drives that
    /// are members of the same group are accounted for together. Use
    /// this option to prevent guests from circumventing throttling
    /// limits by using many small disks instead of a single larger
    /// disk.
    pub group: Option<ShellString>,

    pub throttling_bps_total: Option<u64>,
    pub throttling_bps_total_max: Option<u64>,
    pub throttling_bps_total_max_length: Option<u64>,
    pub throttling_iops_total: Option<u64>,
    pub throttling_iops_total_max: Option<u64>,
    pub throttling_iops_total_max_length: Option<u64>,
}

impl ToCommand for Drive {
    fn command(&self) -> String {
        ARG_DRIVE.to_string()
    }
    fn has_args(&self) -> bool {
        self.discard.is_some()
            || self.read_only.is_some()
            || self.detect_zeroes.is_some()
            || self.file.is_some()
            || self.interface.is_some()
            || self.bus.is_some()
            || self.unit.is_some()
            || self.index.is_some()
            || self.media.is_some()
            || self.snapshot.is_some()
            || self.cache.is_some()
            || self.cache_direct.is_some()
            || self.id.is_some()
            || self.aio.is_some()
            || self.format.is_some()
            || self.encrypt_format.is_some()
            || self.encrypt_key_secret.is_some()
            || self.rerror.is_some()
            || self.werror.is_some()
            || self.copy_on_read.is_some()
            || self.auto_read_only.is_some()
            || self.bps.is_some()
            || self.bps_rd.is_some()
            || self.bps_wr.is_some()
            || self.bps_max.is_some()
            || self.bps_rd_max.is_some()
            || self.bps_wr_max.is_some()
            || self.iops.is_some()
            || self.iops_rd.is_some()
            || self.iops_wr.is_some()
            || self.iops_max.is_some()
            || self.iops_rd_max.is_some()
            || self.iops_wr_max.is_some()
            || self.iops_size.is_some()
            || self.group.is_some()
            || self.throttling_bps_total.is_some()
            || self.throttling_bps_total_max.is_some()
            || self.throttling_bps_total_max_length.is_some()
            || self.throttling_iops_total.is_some()
            || self.throttling_iops_total_max.is_some()
            || self.throttling_iops_total_max_length.is_some()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];

        if let Some(file) = &self.file {
            args.push(format!("{}{}", KEY_FILE, file.as_ref()));
        }
        qao!(&self.interface, args, KEY_INTERFACE);
        qao!(&self.bus, args, KEY_BUS);
        qao!(&self.unit, args, KEY_UNIT);
        if let Some(index) = &self.index {
            args.push(format!("{}{}", KEY_INDEX, index.as_ref()));
        }
        qao!(&self.media, args, KEY_MEDIA);
        qao!(&self.snapshot, args, KEY_SNAPSHOT);
        qao!(&self.cache, args, KEY_CACHE);
        qao!(&self.cache_direct, args, KEY_CACHE_DIRECT);
        qao!(&self.aio, args, KEY_AIO);
        if let Some(format) = &self.format {
            args.push(format!("{}{}", KEY_FORMAT, format.as_ref()));
        }
        if let Some(encrypt_format) = &self.encrypt_format {
            args.push(format!("{}{}", KEY_ENCRYPT_FORMAT, encrypt_format.as_ref()));
        }
        if let Some(encrypt_key_secret) = &self.encrypt_key_secret {
            args.push(format!("{}{}", KEY_ENCRYPT_KEY_SECRET, encrypt_key_secret.as_ref()));
        }
        qao!(&self.rerror, args, KEY_RERROR);
        qao!(&self.werror, args, KEY_WERROR);
        if let Some(id) = &self.id {
            args.push(format!("{}{}", KEY_ID, id.as_ref()));
        }
        qao!(&self.read_only, args, KEY_READ_ONLY);
        qao!(&self.auto_read_only, args, KEY_AUTO_READ_ONLY);
        qao!(&self.copy_on_read, args, KEY_COPY_ON_READ);
        qao!(&self.discard, args, KEY_DISCARD);
        qao!(&self.detect_zeroes, args, KEY_DETECT_ZEROES);
        qao!(&self.bps, args, KEY_BPS);
        qao!(&self.bps_rd, args, KEY_BPS_RD);
        qao!(&self.bps_wr, args, KEY_BPS_WR);
        qao!(&self.bps_max, args, KEY_BPS_MAX);
        qao!(&self.bps_rd_max, args, KEY_BPS_RD_MAX);
        qao!(&self.bps_wr_max, args, KEY_BPS_WR_MAX);
        qao!(&self.iops, args, KEY_IOPS);
        qao!(&self.iops_rd, args, KEY_IOPS_RD);
        qao!(&self.iops_wr, args, KEY_IOPS_WR);
        qao!(&self.iops_max, args, KEY_IOPS_MAX);
        qao!(&self.iops_rd_max, args, KEY_IOPS_RD_MAX);
        qao!(&self.iops_wr_max, args, KEY_IOPS_WR_MAX);
        qao!(&self.iops_size, args, KEY_IOPS_SIZE);
        if let Some(group) = &self.group {
            args.push(format!("{}{}", KEY_GROUP, group.as_ref()));
        }
        qao!(&self.throttling_bps_total, args, KEY_THROTTLING_BPS_TOTAL);
        qao!(&self.throttling_bps_total_max, args, KEY_THROTTLING_BPS_TOTAL_MAX);
        qao!(&self.throttling_bps_total_max_length, args, KEY_THROTTLING_BPS_TOTAL_MAX_LENGTH);
        qao!(&self.throttling_iops_total, args, KEY_THROTTLING_IOPS_TOTAL);
        qao!(&self.throttling_iops_total_max, args, KEY_THROTTLING_IOPS_TOTAL_MAX);
        qao!(&self.throttling_iops_total_max_length, args, KEY_THROTTLING_IOPS_TOTAL_MAX_LENGTH);

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Drive {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut drive = Drive::default();

        for part in s.split(DELIM_COMMA) {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid drive option: {part}"))?;
            match key {
                "file" => drive.file = Some(ShellPath::from(value)),
                "if" => drive.interface = Some(value.parse::<DriveInterface>().map_err(|_| format!("invalid if value: {value}"))?),
                "bus" => drive.bus = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "unit" => drive.unit = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "index" => drive.index = Some(ShellString::from_str(value)?),
                "media" => drive.media = Some(value.parse::<DriveMedia>().map_err(|_| format!("invalid media value: {value}"))?),
                "snapshot" => drive.snapshot = Some(value.parse::<OnOff>().map_err(|_| format!("invalid snapshot value: {value}"))?),
                "cache" => drive.cache = Some(value.parse::<DriveCacheType>().map_err(|_| format!("invalid cache value: {value}"))?),
                "cache.direct" => drive.cache_direct = Some(value.parse::<OnOff>().map_err(|_| format!("invalid cache.direct value: {value}"))?),
                "aio" => drive.aio = Some(value.parse::<DriveAIOType>().map_err(|_| format!("invalid aio value: {value}"))?),
                "format" => drive.format = Some(ShellString::from_str(value)?),
                "encrypt.format" => drive.encrypt_format = Some(ShellString::from_str(value)?),
                "encrypt.key-secret" => drive.encrypt_key_secret = Some(ShellString::from_str(value)?),
                "rerror" => drive.rerror = Some(value.parse::<DriveErrorAction>().map_err(|_| format!("invalid rerror value: {value}"))?),
                "werror" => drive.werror = Some(value.parse::<DriveErrorAction>().map_err(|_| format!("invalid werror value: {value}"))?),
                "id" => drive.id = Some(ShellString::from_str(value)?),
                "readonly" | "read-only" => drive.read_only = Some(value.parse::<OnOff>().map_err(|_| format!("invalid readonly value: {value}"))?),
                "auto-read-only" => drive.auto_read_only = Some(value.parse::<OnOff>().map_err(|_| format!("invalid auto-read-only value: {value}"))?),
                "copy-on-read" => drive.copy_on_read = Some(value.parse::<OnOff>().map_err(|_| format!("invalid copy-on-read value: {value}"))?),
                "discard" => drive.discard = Some(value.parse::<IgnoreUnmap>().map_err(|_| format!("invalid discard value: {value}"))?),
                "detect-zeroes" => drive.detect_zeroes = Some(value.parse::<OnOffUnmap>().map_err(|_| format!("invalid detect-zeroes value: {value}"))?),
                "bps" => drive.bps = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "bps_rd" => drive.bps_rd = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "bps_wr" => drive.bps_wr = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "bps_max" => drive.bps_max = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "bps_rd_max" => drive.bps_rd_max = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "bps_wr_max" => drive.bps_wr_max = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "iops" => drive.iops = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "iops_rd" => drive.iops_rd = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "iops_wr" => drive.iops_wr = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "iops_max" => drive.iops_max = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "iops_rd_max" => drive.iops_rd_max = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "iops_wr_max" => drive.iops_wr_max = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "iops_size" => drive.iops_size = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "group" => drive.group = Some(ShellString::from_str(value)?),
                "throttling.bps-total" => drive.throttling_bps_total = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "throttling.bps-total-max" => drive.throttling_bps_total_max = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "throttling.bps-total-max-length" => drive.throttling_bps_total_max_length = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "throttling.iops-total" => drive.throttling_iops_total = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "throttling.iops-total-max" => drive.throttling_iops_total_max = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                "throttling.iops-total-max-length" => drive.throttling_iops_total_max_length = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
                other => return Err(format!("unsupported drive option: {other}")),
            }
        }

        Ok(drive)
    }
}
