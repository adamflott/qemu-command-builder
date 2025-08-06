use bon::Builder;

use crate::to_command::ToCommand;

#[derive(Default, Builder)]
pub struct NUMANodeMem {
    mem_size: Option<usize>,
    cpu_first: Option<usize>,
    cpu_last: Option<usize>,
    node_id: Option<usize>,
    initiator: Option<usize>,
}

#[derive(Default, Builder)]
pub struct NUMANodeMemDev {
    mem_id: Option<usize>,
    cpu_first: Option<usize>,
    cpu_last: Option<usize>,
    node_id: Option<usize>,
    initiator: Option<usize>,
}
#[derive(Default, Builder)]
pub struct NUMADist {
    src: usize,
    dst: usize,
    val: usize,
}

#[derive(Default, Builder)]
pub struct NUMACPU {
    node_id: usize,
    socket_id: Option<usize>,
    core_id: Option<usize>,
    thread_id: Option<usize>,
}

pub enum NUMAHierarchy {
    Memory,
    FirstLevel,
    SecondLevel,
    ThirdLevel,
}
pub enum NUMADataType {
    AccessLatency,
    ReadLatency,
    WriteLatency,
}

#[derive(Builder)]
pub struct NUMAHMATLb {
    initiator: usize,
    target: usize,
    hierarchy: NUMAHierarchy,
    data_type: NUMADataType,
    latency: Option<usize>,   // TODO nanoseconds type
    bandwidth: Option<usize>, // TODO add value the possible value and units are NUM[M|G|T] mean that the bandwidth value are NUM byte per second (or MB/s, GB/s or TB/s depending on used suffix). Note that if latency or bandwidth value is 0, means the corresponding latency or bandwidth information is not provided.
}

pub enum HMATCacheAssociativity {
    None,
    Direct,
    Complex,
}
pub enum HMATCachePolicy {
    None,
    WriteBack,
    WriteThrough,
}

#[derive(Builder)]
pub struct NUMAHMATCache {
    node_id: usize,
    size: usize,
    level: usize,
    associativity: Option<HMATCacheAssociativity>,
    policy: Option<HMATCachePolicy>,
    line: Option<usize>,
}

pub enum NUMA {
    NodeMem(NUMANodeMem),
    NodeMemDev(NUMANodeMemDev),
    Dist(NUMADist),
    Cpu(NUMACPU),
    HMATLB(NUMAHMATLb),
    HMATCache(NUMAHMATCache),
}

impl ToCommand for NUMA {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec!["-numa".to_string()];

        match self {
            NUMA::NodeMem(node_mem) => {
                let mut node_mem_args = "node".to_string();
                if let Some(mem) = &node_mem.mem_size {
                    node_mem_args.push_str(format!(",mem={}", mem).as_str());
                }
                if let Some(cpu) = &node_mem.cpu_first {
                    node_mem_args.push_str(format!(",cpu={}", cpu).as_str());
                }
                if let Some(cpu) = &node_mem.cpu_last {
                    node_mem_args.push_str(format!("-{}", cpu).as_str());
                }
                if let Some(node_id) = &node_mem.node_id {
                    node_mem_args.push_str(format!(",nodeid={}", node_id).as_str());
                }
                if let Some(initiator) = &node_mem.initiator {
                    node_mem_args.push_str(format!(",initiator={}", initiator).as_str());
                }
                cmd.push(node_mem_args.to_string());
            }
            NUMA::NodeMemDev(node_memdev) => {
                let mut node_memdev_args = "node".to_string();
                if let Some(memdev) = &node_memdev.mem_id {
                    node_memdev_args.push_str(format!(",memdev={}", memdev).as_str());
                }
                if let Some(cpu) = &node_memdev.cpu_first {
                    node_memdev_args.push_str(format!(",cpu={}", cpu).as_str());
                }
                if let Some(cpu) = &node_memdev.cpu_last {
                    node_memdev_args.push_str(format!("-{}", cpu).as_str());
                }
                if let Some(node_id) = &node_memdev.node_id {
                    node_memdev_args.push_str(format!(",nodeid={}", node_id).as_str());
                }
                if let Some(initiator) = &node_memdev.initiator {
                    node_memdev_args.push_str(format!(",initiator={}", initiator).as_str());
                }
                cmd.push(node_memdev_args.to_string());
            }
            NUMA::Dist(dist) => {
                cmd.push(format!(
                    "dist,src={},dst={},val={}",
                    dist.src, dist.dst, dist.val
                ));
            }
            NUMA::Cpu(cpu) => {
                let mut cpu_args = "cpu".to_string();

                cpu_args.push_str(format!(",node-id={}", cpu.node_id).as_str());
                if let Some(socket_id) = &cpu.socket_id {
                    cpu_args.push_str(format!(",socket-id={}", socket_id).as_str());
                }
                if let Some(core_id) = &cpu.core_id {
                    cpu_args.push_str(format!(",core-id={}", core_id).as_str());
                }
                if let Some(thread_id) = &cpu.thread_id {
                    cpu_args.push_str(format!(",thread-id={}", thread_id).as_str());
                }
                cmd.push(cpu_args.to_string());
            }
            NUMA::HMATLB(hmat_lb) => {
                let mut hmat_lb_args = "hmat-lb".to_string();
                hmat_lb_args.push_str(
                    format!(
                        ",initiator={},target={},hierarchy=",
                        hmat_lb.initiator, hmat_lb.target
                    )
                    .as_str(),
                );
                match hmat_lb.hierarchy {
                    NUMAHierarchy::Memory => hmat_lb_args.push_str("memory"),
                    NUMAHierarchy::FirstLevel => hmat_lb_args.push_str("first-level"),
                    NUMAHierarchy::SecondLevel => hmat_lb_args.push_str("second-level"),
                    NUMAHierarchy::ThirdLevel => hmat_lb_args.push_str("third-level"),
                }
                hmat_lb_args.push_str("data-type=");
                match hmat_lb.data_type {
                    NUMADataType::AccessLatency => hmat_lb_args.push_str("access-latency"),
                    NUMADataType::ReadLatency => hmat_lb_args.push_str("read-latency"),
                    NUMADataType::WriteLatency => hmat_lb_args.push_str("write-latency"),
                }
                if let Some(lat) = &hmat_lb.latency {
                    hmat_lb_args.push_str(format!(",latency={}", lat).as_str());
                }
                if let Some(bw) = &hmat_lb.bandwidth {
                    hmat_lb_args.push_str(format!(",bandwidth={}", bw).as_str());
                }
                cmd.push(hmat_lb_args);
            }
            NUMA::HMATCache(hmat_cache) => {
                let mut hmat_cache_args = "hmat-cache".to_string();
                hmat_cache_args.push_str(
                    format!(
                        ",node-id={},size={},level={}",
                        hmat_cache.node_id, hmat_cache.size, hmat_cache.level
                    )
                    .as_str(),
                );
                if let Some(assoc) = &hmat_cache.associativity {
                    hmat_cache_args.push_str(",associativity=");
                    match assoc {
                        HMATCacheAssociativity::None => hmat_cache_args.push_str("none"),
                        HMATCacheAssociativity::Direct => hmat_cache_args.push_str("direct"),
                        HMATCacheAssociativity::Complex => hmat_cache_args.push_str("complex"),
                    }
                }
                if let Some(policy) = &hmat_cache.policy {
                    hmat_cache_args.push_str(",policy=");
                    match policy {
                        HMATCachePolicy::None => hmat_cache_args.push_str("none"),
                        HMATCachePolicy::WriteBack => hmat_cache_args.push_str("write-back"),
                        HMATCachePolicy::WriteThrough => hmat_cache_args.push_str("write-through"),
                    }
                }
                if let Some(line) = &hmat_cache.line {
                    hmat_cache_args.push_str(format!(",line={}", line).as_str());
                }
            }
        }
        cmd
    }
}
