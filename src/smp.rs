use bon::Builder;

use crate::to_command::ToCommand;

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
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder)]
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
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-smp".to_string());

        let mut arg = vec![self.cpus.to_string()];
        if let Some(maxcpus) = self.maxcpus {
            arg.push(format!("maxcpus={}", maxcpus));
        }
        if let Some(drawers) = self.drawers {
            arg.push(format!("drawers={}", drawers));
        }
        if let Some(books) = self.books {
            arg.push(format!("books={}", books));
        }
        if let Some(sockets) = self.sockets {
            arg.push(format!("sockets={}", sockets));
        }
        if let Some(dies) = self.dies {
            arg.push(format!("dies={}", dies));
        }
        if let Some(clusters) = self.clusters {
            arg.push(format!("clusters={}", clusters));
        }
        if let Some(modules) = self.modules {
            arg.push(format!("modules={}", modules));
        }
        if let Some(cores) = self.cores {
            arg.push(format!("cores={}", cores));
        }
        if let Some(threads) = self.threads {
            arg.push(format!("threads={}", threads));
        }

        cmd.push(arg.join(","));

        cmd
    }
}
