use crate::parsers::DELIM_COMMA;
use crate::shell_string::ShellStringError;
use crate::to_command::ToCommand;
use crate::{pco, qao};
use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;
use winnow::ascii::{dec_uint, digit1};
use winnow::combinator::opt;
use winnow::token::literal;
use winnow::{ModalResult, Parser};

pub(crate) const ARG_SMP: &str = "-smp";

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
    cpus: u64,
    /// maximum number of total CPUs, including offline CPUs for hotplug, etc
    maxcpus: Option<usize>,
    /// number of drawers on the machine board
    drawers: Option<usize>,
    /// number of books in one drawer
    books: Option<usize>,
    /// number of sockets in one book
    sockets: Option<usize>,
    /// number of dies in one socket
    dies: Option<usize>,
    /// number of clusters in one die
    clusters: Option<usize>,
    /// number of modules in one cluster
    modules: Option<usize>,
    /// number of cores in one module
    cores: Option<usize>,
    /// number of threads in one core
    threads: Option<usize>,
}

impl Default for SMP {
    fn default() -> Self {
        SMP::new(1)
    }
}

impl SMP {
    pub fn new(cpus: u64) -> Self {
        Self {
            cpus,
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
        let mut args = vec![self.cpus.to_string()];

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
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        smp.parse(s).map_err(|e| ShellStringError::from_parse(e))
    }
}

pco!(maxcpus, digit1, usize, KEY_MAXCPUS);
pco!(drawers, digit1, usize, KEY_DRAWERS);
pco!(books, digit1, usize, KEY_BOOKS);
pco!(sockets, digit1, usize, KEY_SOCKETS);
pco!(dies, digit1, usize, KEY_DIES);
pco!(clusters, digit1, usize, KEY_CLUSTERS);
pco!(modules, digit1, usize, KEY_MODULES);
pco!(cores, digit1, usize, KEY_CORES);
pco!(threads, digit1, usize, KEY_THREADS);

fn smp(s: &mut &str) -> ModalResult<SMP> {
    let cpus = dec_uint.parse_next(s)?;
    let maxcpus = opt(maxcpus).parse_next(s)?;
    let drawers = opt(drawers).parse_next(s)?;
    let books = opt(books).parse_next(s)?;
    let sockets = opt(sockets).parse_next(s)?;
    let dies = opt(dies).parse_next(s)?;
    let clusters = opt(clusters).parse_next(s)?;
    let modules = opt(modules).parse_next(s)?;
    let cores = opt(cores).parse_next(s)?;
    let threads = opt(threads).parse_next(s)?;
    Ok(SMP {
        cpus,
        maxcpus,
        drawers,
        books,
        sockets,
        dies,
        clusters,
        modules,
        cores,
        threads,
    })
}
