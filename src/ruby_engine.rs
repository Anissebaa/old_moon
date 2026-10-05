use crate::analyzer::{PacketAnalyzer, PacketInfo};
use magnus::{eval, prelude::*, RHash, Ruby, Value};
use std::error::Error as StdError;

pub struct RubyEngine {
    _ruby: Ruby,
}

impl RubyEngine {
    pub fn new(script_path: &str) -> Result<Self, Box<dyn StdError>> {
        let ruby = Ruby::get().expect("Ruby not initialised");
        let code = std::fs::read_to_string(script_path)?;
        ruby.eval::<()>(&code)?;
        Ok(Self { _ruby: ruby })
    }
}

impl PacketAnalyzer for RubyEngine {
    fn analyze(&self, pkt: &PacketInfo) -> Result<(), Box<dyn StdError>> {
        let ruby = Ruby::get().expect("Ruby");
        let analyze: Value = ruby.eval("method(:analyze_packet)")?;
        let hash = ruby.hash_new();
        hash.aset("eth_src", pkt.eth_src.clone())?;
        hash.aset("eth_dst", pkt.eth_dst.clone())?;
        hash.aset("eth_type", pkt.eth_type.clone())?;
        hash.aset("src_ip", pkt.src_ip.clone())?;
        hash.aset("dst_ip", pkt.dst_ip.clone())?;
        hash.aset("ttl", pkt.ttl)?;
        hash.aset("length", pkt.length)?;
        hash.aset("src_port", pkt.src_port)?;
        hash.aset("dst_port", pkt.dst_port)?;
        hash.aset("protocol", pkt.protocol.clone())?;
        hash.aset("icmp_type", pkt.icmp_type)?;
        hash.aset("icmp_code", pkt.icmp_code)?;
        hash.aset("arp_op", pkt.arp_op.clone())?;
        hash.aset("arp_sender_ip", pkt.arp_sender_ip.clone())?;
        hash.aset("arp_target_ip", pkt.arp_target_ip.clone())?;
        hash.aset("arp_sender_mac", pkt.arp_sender_mac.clone())?;
        hash.aset("arp_target_mac", pkt.arp_target_mac.clone())?;
        hash.aset("payload", pkt.payload.clone())?;
        analyze.funcall::<_, _, Value>("call", (hash,))?;
        Ok(())
    }
}
