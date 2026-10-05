use crate::analyzer::{PacketAnalyzer, PacketInfo};
use mlua::{Function, Lua, Table};
use std::error::Error;

pub struct LuaEngine {
    lua: Lua,
    analyze: Function,
}

impl LuaEngine {
    pub fn new(script_path: &str) -> Result<Self, Box<dyn Error>> {
        let lua = Lua::new();
        let script = std::fs::read_to_string(script_path)?;
        lua.load(&script).exec()?;
        let analyze: Function = lua.globals().get("analyze_packet")?;
        Ok(Self { lua, analyze })
    }

    fn to_lua_table(&self, pkt: &PacketInfo) -> Result<Table, mlua::Error> {
        let t = self.lua.create_table()?;
        t.set("eth_src",  pkt.eth_src.clone())?;
        t.set("eth_dst",  pkt.eth_dst.clone())?;
        t.set("eth_type", pkt.eth_type.clone())?;
        t.set("src_ip",   pkt.src_ip.clone())?;
        t.set("dst_ip",   pkt.dst_ip.clone())?;
        t.set("ttl",      pkt.ttl)?;
        t.set("length",   pkt.length)?;
        t.set("src_port", pkt.src_port)?;
        t.set("dst_port", pkt.dst_port)?;
        t.set("protocol", pkt.protocol.clone())?;
        t.set("icmp_type", pkt.icmp_type)?;
        t.set("icmp_code", pkt.icmp_code)?;
        t.set("arp_op",         pkt.arp_op.clone())?;
        t.set("arp_sender_ip",  pkt.arp_sender_ip.clone())?;
        t.set("arp_target_ip",  pkt.arp_target_ip.clone())?;
        t.set("arp_sender_mac", pkt.arp_sender_mac.clone())?;
        t.set("arp_target_mac", pkt.arp_target_mac.clone())?;
        t.set("payload", pkt.payload.clone())?;
        Ok(t)
    }
}

impl PacketAnalyzer for LuaEngine {
    fn analyze(&self, pkt: &PacketInfo) -> Result<(), Box<dyn Error>> {
        let table = self.to_lua_table(pkt)?;
        self.analyze.call::<_, ()>(table)?;
        Ok(())
    }
}
