use crate::parsers::ascii_plus_more;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::common::{IgnoreUnmap, OnOff, OnOffUnmap};
use crate::parsers::DELIM_COMMA;
use crate::shell_path::{ShellPath, shell_path_until_comma};
use crate::shell_string::{ShellString, ShellStringError, shell_string_until_comma};
use crate::to_command::{ToArg, ToCommand};
use crate::{pco0, ppo0, qao};
use winnow::Result;
use winnow::ascii::{alphanumeric1, dec_uint};
use winnow::combinator::{fail, opt};
use winnow::prelude::*;
use winnow::token::{literal, take_while};

pub(crate) const ARG_DRIVE: &str = "-drive";

const KEY_FILE: &str = "file=";
const KEY_INTERFACE: &str = "if=";
const KEY_BUS: &str = "bus=";
const KEY_UNIT: &str = "unit=";
const KEY_INDEX: &str = "index=";
const KEY_MEDIA: &str = "media=";
const KEY_SNAPSHOT: &str = "snapshot=";
const KEY_CACHE: &str = "cache=";
const KEY_ID: &str = "id=";
const KEY_AIO: &str = "aio=";
const KEY_FORMAT: &str = "format=";
const KEY_RERROR: &str = "rerror=";
const KEY_WERROR: &str = "werror=";
const KEY_COPY_ON_READY: &str = "copy-on-ready=";
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
const KEY_NODE_NAME: &str = "node-name=";
const KEY_DISCARD: &str = "discard=";
const KEY_CACHE_DIRECT: &str = "cache.direct=";
const KEY_CACHE_NO_FLUSH: &str = "cache.no-flush=";
const KEY_READ_ONLY: &str = "read-only=";
const KEY_AUTO_READ_ONLY: &str = "auto-read-only=";
const KEY_FORCE_SHARE: &str = "force-share=";
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
    pub bus: Option<usize>,
    pub unit: Option<usize>,

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

    /// aio is "threads", "native", or "io_uring" and selects between pthread
    /// based disk I/O, native Linux AIO, or Linux io_uring API.
    pub aio: Option<DriveAIOType>,

    /// Specify which disk format will be used rather than detecting the
    /// format. Can be used to specify format=raw to avoid interpreting
    /// an untrusted format header.
    pub format: Option<ShellString>,

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
    /// the default value ``read-only=off`` does not work and the
    /// option must be specified explicitly.
    pub read_only: Option<OnOff>,

    /// copy-on-read is "on" or "off" and enables whether to copy read
    /// backing file sectors into the image file.
    pub copy_on_ready: Option<OnOff>,

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
    pub bps: Option<usize>,
    pub bps_rd: Option<usize>,
    pub bps_wr: Option<usize>,

    /// Specify bursts in bytes per second, either for all request types
    /// or for reads or writes only. Bursts allow the guest I/O to spike
    /// above the limit temporarily.
    pub bps_max: Option<usize>,
    pub bps_rd_max: Option<usize>,
    pub bps_wr_max: Option<usize>,

    /// Specify request rate limits in requests per second, either for
    /// all request types or for reads or writes only.
    pub iops: Option<usize>,
    pub iops_rd: Option<usize>,
    pub iops_wr: Option<usize>,

    /// Specify bursts in requests per second, either for all request
    /// types or for reads or writes only. Bursts allow the guest I/O to
    /// spike above the limit temporarily.
    pub iops_max: Option<usize>,
    pub iops_rd_max: Option<usize>,
    pub iops_wr_max: Option<usize>,

    /// Let every is bytes of a request count as a new request for iops
    /// throttling purposes. Use this option to prevent guests from
    /// circumventing iops limits by sending fewer but larger requests.
    pub iops_size: Option<usize>,

    /// Join a throttling quota group with given name g. All drives that
    /// are members of the same group are accounted for together. Use
    /// this option to prevent guests from circumventing throttling
    /// limits by using many small disks instead of a single larger
    /// disk.
    pub group: Option<ShellString>,
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
            || self.id.is_some()
            || self.aio.is_some()
            || self.format.is_some()
            || self.rerror.is_some()
            || self.werror.is_some()
            || self.copy_on_ready.is_some()
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
        qao!(&self.aio, args, KEY_AIO);
        if let Some(format) = &self.format {
            args.push(format!("{}{}", KEY_FORMAT, format.as_ref()));
        }
        qao!(&self.rerror, args, KEY_RERROR);
        qao!(&self.werror, args, KEY_WERROR);
        if let Some(id) = &self.id {
            args.push(format!("{}{}", KEY_ID, id.as_ref()));
        }
        qao!(&self.read_only, args, KEY_READ_ONLY);
        qao!(&self.copy_on_ready, args, KEY_COPY_ON_READY);
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

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Drive {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        drive.parse(s).map_err(|e| ShellStringError::from_parse(e))
    }
}

pco0!(file, shell_path_until_comma, ShellPath, KEY_FILE);
pco0!(interface, alphanumeric1, DriveInterface, KEY_INTERFACE);
ppo0!(bus, dec_uint, usize, KEY_BUS);
ppo0!(unit, dec_uint, usize, KEY_UNIT);
pco0!(index, shell_string_until_comma, ShellString, KEY_INDEX);
pco0!(media, alphanumeric1, DriveMedia, KEY_MEDIA);
pco0!(snapshot, alphanumeric1, OnOff, KEY_SNAPSHOT);
pco0!(cache, alphanumeric1, DriveCacheType, KEY_CACHE);
pco0!(aio, ascii_plus_more, DriveAIOType, KEY_AIO);
pco0!(format, shell_string_until_comma, ShellString, KEY_FORMAT);
pco0!(rerror, alphanumeric1, DriveErrorAction, KEY_RERROR);
pco0!(werror, alphanumeric1, DriveErrorAction, KEY_WERROR);
pco0!(id, shell_string_until_comma, ShellString, KEY_ID);
pco0!(read_only, alphanumeric1, OnOff, KEY_READ_ONLY);
pco0!(copy_on_ready, alphanumeric1, OnOff, KEY_COPY_ON_READY);
pco0!(discard, alphanumeric1, IgnoreUnmap, KEY_DISCARD);
pco0!(detect_zeroes, alphanumeric1, OnOffUnmap, KEY_DETECT_ZEROES);
ppo0!(bps, dec_uint, usize, KEY_BPS);
ppo0!(bps_rd, dec_uint, usize, KEY_BPS_RD);
ppo0!(bps_wr, dec_uint, usize, KEY_BPS_WR);
ppo0!(bps_max, dec_uint, usize, KEY_BPS_MAX);
ppo0!(bps_rd_max, dec_uint, usize, KEY_BPS_RD_MAX);
ppo0!(bps_wr_max, dec_uint, usize, KEY_BPS_WR_MAX);
ppo0!(iops, dec_uint, usize, KEY_IOPS);
ppo0!(iops_rd, dec_uint, usize, KEY_IOPS_RD);
ppo0!(iops_wr, dec_uint, usize, KEY_IOPS_WR);
ppo0!(iops_max, dec_uint, usize, KEY_IOPS_MAX);
ppo0!(iops_rd_max, dec_uint, usize, KEY_IOPS_RD_MAX);
ppo0!(iops_wr_max, dec_uint, usize, KEY_IOPS_WR_MAX);
ppo0!(iops_size, dec_uint, usize, KEY_IOPS_SIZE);
pco0!(group, shell_string_until_comma, ShellString, KEY_GROUP);

pub fn drive2(s: &mut &str) -> ModalResult<Drive> {
    //let ks = [KEY_FILE, KEY_INTERFACE, KEY_BUS, KEY_UNIT, KEY_INDEX, KEY_MEDIA].map(|v|literal(v)).collect();
    //let k = alt( (literal(KEY_FILE), literal(KEY_INTERFACE), literal(KEY_BUS)) ).parse(s)?;
    let k = take_while(1.., |c: char| c != '=').parse_next(s)?;
    let mut d = Drive::builder().build();
    match k {
        KEY_FILE => {
            let file = ascii_plus_more.parse_next(s)?;
            d.file = Some(ShellPath { s: file.to_string() });
        }
        KEY_INTERFACE => {}
        KEY_BUS => {}
        KEY_UNIT => {}
        KEY_INDEX => {}
        KEY_MEDIA => {}
        KEY_SNAPSHOT => {}
        KEY_CACHE => {}
        KEY_ID => {}
        KEY_AIO => {}
        KEY_FORMAT => {}
        KEY_RERROR => {}
        KEY_WERROR => {}
        KEY_COPY_ON_READY => {}
        KEY_BPS => {}
        KEY_BPS_RD => {}
        KEY_BPS_WR => {}
        KEY_BPS_MAX => {}
        KEY_BPS_RD_MAX => {}
        KEY_BPS_WR_MAX => {}
        KEY_IOPS => {}
        KEY_IOPS_RD => {}
        KEY_IOPS_WR => {}
        KEY_IOPS_MAX => {}
        KEY_IOPS_RD_MAX => {}
        KEY_IOPS_WR_MAX => {}
        KEY_IOPS_SIZE => {}
        KEY_GROUP => {}
        KEY_NODE_NAME => {}
        KEY_DISCARD => {}
        KEY_CACHE_DIRECT => {}
        KEY_CACHE_NO_FLUSH => {}
        KEY_READ_ONLY => {}
        KEY_AUTO_READ_ONLY => {}
        KEY_FORCE_SHARE => {}
        KEY_DETECT_ZEROES => {}
        _ => return fail(s),
    }

    todo!()
}
pub fn drive(s: &mut &str) -> ModalResult<Drive> {
    let file = opt(file).parse_next(s)?;
    let interface = opt(interface).parse_next(s)?;
    let bus = opt(bus).parse_next(s)?;
    let unit = opt(unit).parse_next(s)?;
    let index = opt(index).parse_next(s)?;
    let media = opt(media).parse_next(s)?;
    let snapshot = opt(snapshot).parse_next(s)?;
    let cache = opt(cache).parse_next(s)?;
    let aio = opt(aio).parse_next(s)?;
    let format = opt(format).parse_next(s)?;
    let rerror = opt(rerror).parse_next(s)?;
    let werror = opt(werror).parse_next(s)?;
    let id = opt(id).parse_next(s)?;
    let read_only = opt(read_only).parse_next(s)?;
    let copy_on_ready = opt(copy_on_ready).parse_next(s)?;
    let discard = opt(discard).parse_next(s)?;
    let detect_zeroes = opt(detect_zeroes).parse_next(s)?;
    let bps = opt(bps).parse_next(s)?;
    let bps_rd = opt(bps_rd).parse_next(s)?;
    let bps_wr = opt(bps_wr).parse_next(s)?;
    let bps_max = opt(bps_max).parse_next(s)?;
    let bps_rd_max = opt(bps_rd_max).parse_next(s)?;
    let bps_wr_max = opt(bps_wr_max).parse_next(s)?;
    let iops = opt(iops).parse_next(s)?;
    let iops_rd = opt(iops_rd).parse_next(s)?;
    let iops_wr = opt(iops_wr).parse_next(s)?;
    let iops_max = opt(iops_max).parse_next(s)?;
    let iops_rd_max = opt(iops_rd_max).parse_next(s)?;
    let iops_wr_max = opt(iops_wr_max).parse_next(s)?;
    let iops_size = opt(iops_size).parse_next(s)?;
    let group = opt(group).parse_next(s)?;
    Ok(Drive {
        file,
        interface,
        bus,
        unit,
        index,
        media,
        snapshot,
        cache,
        aio,
        format,
        rerror,
        werror,
        id,
        read_only,
        copy_on_ready,
        discard,
        detect_zeroes,
        bps,
        bps_rd,
        bps_wr,
        bps_max,
        bps_rd_max,
        bps_wr_max,
        iops,
        iops_rd,
        iops_wr,
        iops_max,
        iops_rd_max,
        iops_wr_max,
        iops_size,
        group,
    })
}
