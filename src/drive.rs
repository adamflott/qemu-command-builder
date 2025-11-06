use std::path::PathBuf;

use bon::Builder;

use crate::common::{IgnoreUnmap, OnOff, OnOffUnmap};
use crate::to_command::{ToArg, ToCommand};

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
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

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum DriveMedia {
    Disk,
    Cdrom,
}

impl ToArg for DriveMedia {
    fn to_arg(&self) -> &str {
        match self {
            DriveMedia::Disk => "disk",
            DriveMedia::Cdrom => "cdrom",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default)]
pub enum DriveCacheType {
    None,
    #[default]
    Writeback,
    Writethrough,
    Unsafe,
    Directsync,
}

impl ToArg for DriveCacheType {
    fn to_arg(&self) -> &str {
        match self {
            DriveCacheType::None => "none",
            DriveCacheType::Writeback => "writeback",
            DriveCacheType::Writethrough => "unsafe",
            DriveCacheType::Unsafe => "directsync",
            DriveCacheType::Directsync => "writethrough",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum DriveAIOType {
    Threads,
    Native,
    IOUring,
}

impl ToArg for DriveAIOType {
    fn to_arg(&self) -> &str {
        match self {
            DriveAIOType::Threads => "threads",
            DriveAIOType::Native => "native",
            DriveAIOType::IOUring => "io_uring",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum DriveErrorAction {
    Ignore,
    Stop,
    Report,
    Enospc,
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
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct Drive {
    // -blockdev
    /// This defines the name of the block driver node by which it
    /// will be referenced later. The name must be unique, i.e. it
    /// must not match the name of a different block driver node, or
    /// (if you use ``-drive`` as well) the ID of a drive.
    ///
    /// If no node name is specified, it is automatically generated.
    /// The generated node name is not intended to be predictable
    /// and changes between QEMU invocations. For the top level, an
    /// explicit node name must be specified.
    pub node_name: Option<String>,

    /// discard is one of "ignore" (or "off") or "unmap" (or "on")
    /// and controls whether ``discard`` (also known as ``trim`` or
    /// ``unmap``) requests are ignored or passed to the filesystem.
    /// Some machine types may not support discard requests.
    pub discard: Option<IgnoreUnmap>,

    /// The host page cache can be avoided with ``cache.direct=on``.
    /// This will attempt to do disk IO directly to the guest's
    /// memory. QEMU may still perform an internal copy of the data.
    pub cache_direct: Option<OnOff>,

    /// In case you don't care about data integrity over host
    /// failures, you can use ``cache.no-flush=on``. This option
    /// tells QEMU that it never needs to write any data to the disk
    /// but can instead keep things in cache. If anything goes
    /// wrong, like your host losing power, the disk storage getting
    /// disconnected accidentally, etc. your image will most
    /// probably be rendered unusable.
    pub cache_no_flush: Option<OnOff>,

    /// Open the node read-only. Guest write attempts will fail.
    ///
    /// Note that some block drivers support only read-only access,
    /// either generally or in certain configurations. In this case,
    /// the default value ``read-only=off`` does not work and the
    /// option must be specified explicitly.
    pub read_only: Option<OnOff>,

    /// If ``auto-read-only=on`` is set, QEMU may fall back to
    /// read-only usage even when ``read-only=off`` is requested, or
    /// even switch between modes as needed, e.g. depending on
    /// whether the image file is writable or whether a writing user
    /// is attached to the node.
    pub auto_read_only: Option<OnOff>,

    /// Override the image locking system of QEMU by forcing the
    /// node to utilize weaker shared access for permissions where
    /// it would normally request exclusive access. When there is
    /// the potential for multiple instances to have the same file
    /// open (whether this invocation of QEMU is the first or the
    /// second instance), both instances must permit shared access
    /// for the second instance to succeed at opening the file.
    ///
    /// Enabling ``force-share=on`` requires ``read-only=on``.
    pub force_share: Option<OnOff>,

    /// detect-zeroes is "off", "on" or "unmap" and enables the
    /// automatic conversion of plain zero writes by the OS to
    /// driver specific optimized zero write commands. You may even
    /// choose "unmap" if discard is set to "unmap" to allow a zero
    /// write to be converted to an ``unmap`` operation.
    pub detect_zeroes: Option<OnOffUnmap>,

    // -drive
    /// This option defines which disk image (see the :ref:`disk images`
    /// chapter in the System Emulation Users Guide) to use with this drive.
    /// If the filename contains comma, you must double it (for instance,
    /// "file=my,,file" to use file "my,file").
    ///
    /// Special files such as iSCSI devices can be specified using
    /// protocol specific URLs. See the section for "Device URL Syntax"
    /// for more information.
    pub file: Option<PathBuf>,

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
    pub index: Option<String>,

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

    pub id: Option<String>,

    /// aio is "threads", "native", or "io_uring" and selects between pthread
    /// based disk I/O, native Linux AIO, or Linux io_uring API.
    pub aio: Option<DriveAIOType>,

    /// Specify which disk format will be used rather than detecting the
    /// format. Can be used to specify format=raw to avoid interpreting
    /// an untrusted format header.
    pub format: Option<String>,

    /// Specify which action to take on write and read errors. Valid
    /// actions are: "ignore" (ignore the error and try to continue),
    /// "stop" (pause QEMU), "report" (report the error to the guest),
    /// "enospc" (pause QEMU only if the host disk is full; report the
    /// error to the guest otherwise). The default setting is
    /// ``werror=enospc`` and ``rerror=report``.
    pub rerror: Option<DriveErrorAction>,
    pub werror: Option<DriveErrorAction>,

    /// copy-on-read is "on" or "off" and enables whether to copy read
    /// backing file sectors into the image file.
    pub copy_on_ready: Option<OnOff>,

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
    pub group: Option<String>,
}

impl ToCommand for Drive {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-drive".to_string());

        let mut args = vec![];

        // -drive
        if let Some(file) = &self.file {
            args.push(format!("file={}", file.display()));
        }
        if let Some(interface) = &self.interface {
            args.push(format!("if={}", interface.to_arg()));
        }
        if let Some(bus) = &self.bus {
            args.push(format!("bus={}", bus));
        }
        if let Some(unit) = &self.unit {
            args.push(format!("unit={}", unit));
        }
        if let Some(index) = &self.index {
            args.push(format!("index={}", index));
        }
        if let Some(media) = &self.media {
            args.push(format!("media={}", media.to_arg()));
        }
        if let Some(snapshot) = &self.snapshot {
            args.push(format!("snapshot={}", snapshot.to_arg()));
        }
        if let Some(cache) = &self.cache {
            args.push(format!("cache={}", cache.to_arg()));
        }
        if let Some(id) = &self.id {
            args.push(format!("id={}", id));
        }
        if let Some(aio) = &self.aio {
            args.push(format!("aio={}", aio.to_arg()));
        }
        if let Some(format) = &self.format {
            args.push(format!("format={}", format));
        }
        if let Some(rerror) = &self.rerror {
            args.push(format!("rerror={}", rerror.to_arg()));
        }
        if let Some(werror) = &self.werror {
            args.push(format!("werror={}", werror.to_arg()));
        }
        if let Some(copy_on_ready) = &self.copy_on_ready {
            args.push(format!("copy-on-ready={}", copy_on_ready.to_arg()));
        }
        if let Some(bps) = &self.bps {
            args.push(format!("bps={}", bps));
        }
        if let Some(bps_rd) = &self.bps_rd {
            args.push(format!("bps_rd={}", bps_rd));
        }
        if let Some(bps_wr) = &self.bps_wr {
            args.push(format!("bps_wr={}", bps_wr));
        }
        if let Some(bps_max) = &self.bps_max {
            args.push(format!("bps_max={}", bps_max));
        }
        if let Some(bps_rd_max) = &self.bps_rd_max {
            args.push(format!("bps_rd_max={}", bps_rd_max));
        }
        if let Some(bps_wr_max) = &self.bps_wr_max {
            args.push(format!("bps_wr_max={}", bps_wr_max));
        }
        if let Some(iops) = self.iops {
            args.push(format!("iops={}", iops));
        }
        if let Some(iops_rd) = &self.iops_rd {
            args.push(format!("iops_rd={}", iops_rd));
        }
        if let Some(iops_wr) = &self.iops_wr {
            args.push(format!("iops_wr={}", iops_wr));
        }
        if let Some(iops_max) = &self.iops_max {
            args.push(format!("iops_max={}", iops_max));
        }
        if let Some(iops_rd_max) = &self.iops_rd_max {
            args.push(format!("iops_rd_max={}", iops_rd_max));
        }
        if let Some(iops_wr_max) = &self.iops_wr_max {
            args.push(format!("iops_wr_max={}", iops_wr_max));
        }
        if let Some(iops_size) = &self.iops_size {
            args.push(format!("iops_size={}", iops_size));
        }
        if let Some(group) = &self.group {
            args.push(format!("group={}", group));
        }

        // -blockdev
        if let Some(node_name) = &self.node_name {
            args.push(format!("node-name={}", node_name));
        }
        if let Some(discard) = &self.discard {
            args.push(format!("discard={}", discard.to_arg()));
        }
        if let Some(cache_direct) = &self.cache_direct {
            args.push(format!("cache.direct={}", cache_direct.to_arg()));
        }
        if let Some(cache_no_flush) = &self.cache_no_flush {
            args.push(format!("cache.no-flush={}", cache_no_flush.to_arg()));
        }
        if let Some(read_only) = &self.read_only {
            args.push(format!("read-only={}", read_only.to_arg()));
        }
        if let Some(auto_read_only) = &self.auto_read_only {
            args.push(format!("auto-read-only={}", auto_read_only.to_arg()));
        }
        if let Some(force_share) = &self.force_share {
            args.push(format!("force-share={}", force_share.to_arg()));
        }
        if let Some(detect_zeroes) = &self.detect_zeroes {
            args.push(format!("detect-zeroes={}", detect_zeroes.to_arg()));
        }

        cmd.push(args.join(","));

        cmd
    }
}
