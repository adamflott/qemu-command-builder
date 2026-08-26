use crate::parsers::ARG_SMP;
use crate::parsers::DELIM_COMMA;
use crate::qao;
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;

const KEY_MAXCPUS: &str = "maxcpus=";
const KEY_DRAWERS: &str = "drawers=";
const KEY_BOOKS: &str = "books=";
const KEY_SOCKETS: &str = "sockets=";
const KEY_DIES: &str = "dies=";
const KEY_CLUSTERS: &str = "clusters=";
const KEY_MODULES: &str = "modules=";
const KEY_CORES: &str = "cores=";
const KEY_THREADS: &str = "threads=";

/// Simulate a SMP system with ``n`` CPUs initially present on
/// the machine type board. On boards supporting CPU hotplug, the optional
/// ``maxcpus`` parameter can be set to enable further CPUs to be
/// added at runtime. When both parameters are omitted, the maximum number
/// of CPUs will be calculated from the provided topology members and the
/// initial CPU count will match the maximum number. When only one of them
/// is given then the omitted one will be set to its counterpart's value.
/// Both parameters may be specified, but the maximum number of CPUs must
/// be equal to or greater than the initial CPU count. Product of the
/// CPU topology hierarchy must be equal to the maximum number of CPUs.
/// Both parameters are subject to an upper limit that is determined by
/// the specific machine type chosen.
///
/// To control reporting of CPU topology information, values of the topology
/// parameters can be specified. Machines may only support a subset of the
/// parameters and different machines may have different subsets supported
/// which vary depending on capacity of the corresponding CPU targets. So
/// for a particular machine type board, an expected topology hierarchy can
/// be defined through the supported sub-option. Unsupported parameters can
/// also be provided in addition to the sub-option, but their values must be
/// set as 1 in the purpose of correct parsing.
///
/// Either the initial CPU count, or at least one of the topology parameters
/// must be specified. The specified parameters must be greater than zero,
/// explicit configuration like "cpus=0" is not allowed. Values for any
/// omitted parameters will be computed from those which are given.
///
/// For example, the following sub-option defines a CPU topology hierarchy
/// (2 sockets totally on the machine, 2 cores per socket, 2 threads per
/// core) for a machine that only supports sockets/cores/threads.
/// Some members of the option can be omitted but their values will be
/// automatically computed:
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct SMP {
    /// set the number of initial CPUs to 'n' [default=1]
    cpus: Option<u64>,
    /// maximum number of total CPUs, including offline CPUs for hotplug, etc
    maxcpus: Option<u64>,
    /// number of drawers on the machine board
    drawers: Option<u64>,
    /// number of books in one drawer
    books: Option<u64>,
    /// number of sockets in one book
    sockets: Option<u64>,
    /// number of dies in one socket
    dies: Option<u64>,
    /// number of clusters in one die
    clusters: Option<u64>,
    /// number of modules in one cluster
    modules: Option<u64>,
    /// number of cores in one module
    cores: Option<u64>,
    /// number of threads in one core
    threads: Option<u64>,
}

impl Default for SMP {
    fn default() -> Self {
        SMP::new(1)
    }
}

impl SMP {
    pub fn new(cpus: u64) -> Self {
        Self {
            cpus: Some(cpus),
            maxcpus: None,
            drawers: None,
            books: None,
            sockets: None,
            dies: None,
            clusters: None,
            modules: None,
            cores: None,
            threads: None,
        }
    }
}

impl ToCommand for SMP {
    fn command(&self) -> String {
        ARG_SMP.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];
        if let Some(cpus) = self.cpus {
            args.push(cpus.to_string());
        }

        qao!(self.maxcpus, args, KEY_MAXCPUS);
        qao!(self.drawers, args, KEY_DRAWERS);
        qao!(self.books, args, KEY_BOOKS);
        qao!(self.sockets, args, KEY_SOCKETS);
        qao!(self.dies, args, KEY_DIES);
        qao!(self.clusters, args, KEY_CLUSTERS);
        qao!(self.modules, args, KEY_MODULES);
        qao!(self.cores, args, KEY_CORES);
        qao!(self.threads, args, KEY_THREADS);

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for SMP {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = Self {
            cpus: None,
            maxcpus: None,
            drawers: None,
            books: None,
            sockets: None,
            dies: None,
            clusters: None,
            modules: None,
            cores: None,
            threads: None,
        };
        for (index, part) in s.split(DELIM_COMMA).enumerate() {
            if index == 0 && !part.contains('=') {
                value.cpus = Some(part.parse::<u64>().map_err(|e| e.to_string())?);
                continue;
            }
            let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid -smp option: {part}"))?;
            macro_rules! number {
                ($field:ident, $type:ty) => {
                    value.$field = Some(raw.parse::<$type>().map_err(|e| e.to_string())?)
                };
            }
            match key {
                "cpus" => number!(cpus, u64),
                "maxcpus" => number!(maxcpus, u64),
                "drawers" => number!(drawers, u64),
                "books" => number!(books, u64),
                "sockets" => number!(sockets, u64),
                "dies" => number!(dies, u64),
                "clusters" => number!(clusters, u64),
                "modules" => number!(modules, u64),
                "cores" => number!(cores, u64),
                "threads" => number!(threads, u64),
                other => return Err(format!("unsupported -smp option: {other}")),
            }
        }
        if value.cpus.is_none()
            && value.drawers.is_none()
            && value.books.is_none()
            && value.sockets.is_none()
            && value.dies.is_none()
            && value.clusters.is_none()
            && value.modules.is_none()
            && value.cores.is_none()
            && value.threads.is_none()
        {
            return Err("-smp requires cpus or at least one topology property".to_string());
        }
        Ok(value)
    }
}
