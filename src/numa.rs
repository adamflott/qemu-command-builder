use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;

use crate::parsers::DELIM_COMMA;
use crate::to_command::ToCommand;

pub(crate) const ARG_NUMA: &str = "-numa";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct NUMANodeMem {
    mem_size: Option<usize>,
    cpu_first: Option<usize>,
    cpu_last: Option<usize>,
    node_id: Option<usize>,
    initiator: Option<usize>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct NUMANodeMemDev {
    mem_id: Option<usize>,
    cpu_first: Option<usize>,
    cpu_last: Option<usize>,
    node_id: Option<usize>,
    initiator: Option<usize>,
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct NUMADist {
    src: usize,
    dst: usize,
    val: usize,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct NUMACPU {
    node_id: usize,
    socket_id: Option<usize>,
    core_id: Option<usize>,
    thread_id: Option<usize>,
}

/// HMAT hierarchy selector for `-numa hmat-lb,...`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum NUMAHierarchy {
    Memory,
    FirstLevel,
    SecondLevel,
    ThirdLevel,
}
/// HMAT latency data type for `-numa hmat-lb,...`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum NUMADataType {
    AccessLatency,
    ReadLatency,
    WriteLatency,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct NUMAHMATLb {
    initiator: usize,
    target: usize,
    hierarchy: NUMAHierarchy,
    data_type: NUMADataType,
    latency: Option<usize>,   // TODO nanoseconds type
    bandwidth: Option<usize>, // TODO add value the possible value and units are NUM[M|G|T] mean that the bandwidth value are NUM byte per second (or MB/s, GB/s or TB/s depending on used suffix). Note that if latency or bandwidth value is 0, means the corresponding latency or bandwidth information is not provided.
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum HMATCacheAssociativity {
    None,
    Direct,
    Complex,
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum HMATCachePolicy {
    None,
    WriteBack,
    WriteThrough,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct NUMAHMATCache {
    node_id: usize,
    size: usize,
    level: usize,
    associativity: Option<HMATCacheAssociativity>,
    policy: Option<HMATCachePolicy>,
    line: Option<usize>,
}

/// A supported `-numa` clause.
///
/// The parser accepts the same comma-separated forms that this crate
/// renders for node, distance, CPU, and HMAT entries.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum NUMA {
    NodeMem(NUMANodeMem),
    NodeMemDev(NUMANodeMemDev),
    Dist(NUMADist),
    Cpu(NUMACPU),
    HMATLB(NUMAHMATLb),
    HMATCache(NUMAHMATCache),
}

impl ToCommand for NUMA {
    fn command(&self) -> String {
        ARG_NUMA.to_string()
    }
    fn to_args(&self) -> Vec<String> {
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
                vec![node_mem_args.to_string()]
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
                vec![node_memdev_args.to_string()]
            }
            NUMA::Dist(dist) => {
                vec![format!("dist,src={},dst={},val={}", dist.src, dist.dst, dist.val)]
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
                vec![cpu_args.to_string()]
            }
            NUMA::HMATLB(hmat_lb) => {
                let mut hmat_lb_args = "hmat-lb".to_string();
                hmat_lb_args.push_str(format!(",initiator={},target={},hierarchy=", hmat_lb.initiator, hmat_lb.target).as_str());
                match hmat_lb.hierarchy {
                    NUMAHierarchy::Memory => hmat_lb_args.push_str("memory"),
                    NUMAHierarchy::FirstLevel => hmat_lb_args.push_str("first-level"),
                    NUMAHierarchy::SecondLevel => hmat_lb_args.push_str("second-level"),
                    NUMAHierarchy::ThirdLevel => hmat_lb_args.push_str("third-level"),
                }
                hmat_lb_args.push_str(",data-type=");
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
                vec![hmat_lb_args]
            }
            NUMA::HMATCache(hmat_cache) => {
                let mut hmat_cache_args = "hmat-cache".to_string();
                hmat_cache_args.push_str(format!(",node-id={},size={},level={}", hmat_cache.node_id, hmat_cache.size, hmat_cache.level).as_str());
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
                vec![hmat_cache_args]
            }
        }
    }
}

impl FromStr for NUMA {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(DELIM_COMMA);
        let kind = parts.next().ok_or_else(|| "empty numa argument".to_string())?;

        match kind {
            "node" => {
                let mut mem_size = None;
                let mut memdev = None;
                let mut cpu_first = None;
                let mut cpu_last = None;
                let mut node_id = None;
                let mut initiator = None;

                for part in parts {
                    let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid numa node option: {part}"))?;
                    match key {
                        "mem" => mem_size = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        "memdev" => memdev = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        "cpu" => {
                            if let Some((first, last)) = raw.split_once('-') {
                                cpu_first = Some(first.parse::<usize>().map_err(|e| e.to_string())?);
                                cpu_last = Some(last.parse::<usize>().map_err(|e| e.to_string())?);
                            } else {
                                cpu_first = Some(raw.parse::<usize>().map_err(|e| e.to_string())?);
                            }
                        }
                        "nodeid" => node_id = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        "initiator" => initiator = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        other => return Err(format!("unsupported numa node option: {other}")),
                    }
                }

                if let Some(mem_id) = memdev {
                    Ok(Self::NodeMemDev(NUMANodeMemDev {
                        mem_id: Some(mem_id),
                        cpu_first,
                        cpu_last,
                        node_id,
                        initiator,
                    }))
                } else {
                    Ok(Self::NodeMem(NUMANodeMem {
                        mem_size,
                        cpu_first,
                        cpu_last,
                        node_id,
                        initiator,
                    }))
                }
            }
            "dist" => {
                let mut src = None;
                let mut dst = None;
                let mut val = None;

                for part in parts {
                    let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid numa dist option: {part}"))?;
                    match key {
                        "src" => src = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        "dst" => dst = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        "val" => val = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        other => return Err(format!("unsupported numa dist option: {other}")),
                    }
                }

                Ok(Self::Dist(NUMADist {
                    src: src.ok_or_else(|| "numa dist requires src=".to_string())?,
                    dst: dst.ok_or_else(|| "numa dist requires dst=".to_string())?,
                    val: val.ok_or_else(|| "numa dist requires val=".to_string())?,
                }))
            }
            "cpu" => {
                let mut node_id = None;
                let mut socket_id = None;
                let mut core_id = None;
                let mut thread_id = None;

                for part in parts {
                    let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid numa cpu option: {part}"))?;
                    match key {
                        "node-id" => node_id = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        "socket-id" => socket_id = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        "core-id" => core_id = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        "thread-id" => thread_id = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        other => return Err(format!("unsupported numa cpu option: {other}")),
                    }
                }

                Ok(Self::Cpu(NUMACPU {
                    node_id: node_id.ok_or_else(|| "numa cpu requires node-id=".to_string())?,
                    socket_id,
                    core_id,
                    thread_id,
                }))
            }
            "hmat-lb" => {
                let mut initiator = None;
                let mut target = None;
                let mut hierarchy = None;
                let mut data_type = None;
                let mut latency = None;
                let mut bandwidth = None;

                for part in parts {
                    let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid numa hmat-lb option: {part}"))?;
                    match key {
                        "initiator" => initiator = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        "target" => target = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        "hierarchy" => hierarchy = Some(parse_numa_hierarchy(raw)?),
                        "data-type" => data_type = Some(parse_numa_data_type(raw)?),
                        "latency" => latency = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        "bandwidth" => bandwidth = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        other => return Err(format!("unsupported numa hmat-lb option: {other}")),
                    }
                }

                Ok(Self::HMATLB(NUMAHMATLb {
                    initiator: initiator.ok_or_else(|| "numa hmat-lb requires initiator=".to_string())?,
                    target: target.ok_or_else(|| "numa hmat-lb requires target=".to_string())?,
                    hierarchy: hierarchy.ok_or_else(|| "numa hmat-lb requires hierarchy=".to_string())?,
                    data_type: data_type.ok_or_else(|| "numa hmat-lb requires data-type=".to_string())?,
                    latency,
                    bandwidth,
                }))
            }
            "hmat-cache" => {
                let mut node_id = None;
                let mut size = None;
                let mut level = None;
                let mut associativity = None;
                let mut policy = None;
                let mut line = None;

                for part in parts {
                    let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid numa hmat-cache option: {part}"))?;
                    match key {
                        "node-id" => node_id = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        "size" => size = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        "level" => level = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        "associativity" => associativity = Some(parse_hmat_cache_associativity(raw)?),
                        "policy" => policy = Some(parse_hmat_cache_policy(raw)?),
                        "line" => line = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                        other => return Err(format!("unsupported numa hmat-cache option: {other}")),
                    }
                }

                Ok(Self::HMATCache(NUMAHMATCache {
                    node_id: node_id.ok_or_else(|| "numa hmat-cache requires node-id=".to_string())?,
                    size: size.ok_or_else(|| "numa hmat-cache requires size=".to_string())?,
                    level: level.ok_or_else(|| "numa hmat-cache requires level=".to_string())?,
                    associativity,
                    policy,
                    line,
                }))
            }
            other => Err(format!("unsupported numa kind: {other}")),
        }
    }
}

fn parse_numa_hierarchy(value: &str) -> Result<NUMAHierarchy, String> {
    match value {
        "memory" => Ok(NUMAHierarchy::Memory),
        "first-level" => Ok(NUMAHierarchy::FirstLevel),
        "second-level" => Ok(NUMAHierarchy::SecondLevel),
        "third-level" => Ok(NUMAHierarchy::ThirdLevel),
        other => Err(format!("invalid numa hierarchy: {other}")),
    }
}

fn parse_numa_data_type(value: &str) -> Result<NUMADataType, String> {
    match value {
        "access-latency" => Ok(NUMADataType::AccessLatency),
        "read-latency" => Ok(NUMADataType::ReadLatency),
        "write-latency" => Ok(NUMADataType::WriteLatency),
        other => Err(format!("invalid numa data-type: {other}")),
    }
}

fn parse_hmat_cache_associativity(value: &str) -> Result<HMATCacheAssociativity, String> {
    match value {
        "none" => Ok(HMATCacheAssociativity::None),
        "direct" => Ok(HMATCacheAssociativity::Direct),
        "complex" => Ok(HMATCacheAssociativity::Complex),
        other => Err(format!("invalid hmat-cache associativity: {other}")),
    }
}

fn parse_hmat_cache_policy(value: &str) -> Result<HMATCachePolicy, String> {
    match value {
        "none" => Ok(HMATCachePolicy::None),
        "write-back" => Ok(HMATCachePolicy::WriteBack),
        "write-through" => Ok(HMATCachePolicy::WriteThrough),
        other => Err(format!("invalid hmat-cache policy: {other}")),
    }
}
