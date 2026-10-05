use std::error::Error;

#[derive(Debug, Clone, Default)]
pub struct PacketInfo {
    pub eth_src: String,
    pub eth_dst: String,
    pub eth_type: String,

    pub src_ip: Option<String>,
    pub dst_ip: Option<String>,
    pub ttl: Option<u8>,
    pub length: u32,

    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub protocol: String,

    pub icmp_type: Option<u8>,
    pub icmp_code: Option<u8>,

    pub arp_op: Option<String>,
    pub arp_sender_ip: Option<String>,
    pub arp_target_ip: Option<String>,
    pub arp_sender_mac: Option<String>,
    pub arp_target_mac: Option<String>,

    pub payload: Vec<u8>,
}

pub trait PacketAnalyzer {
    fn analyze(&self, pkt: &PacketInfo) -> Result<(), Box<dyn Error>>;
}
