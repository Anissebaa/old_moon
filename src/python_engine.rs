use crate::analyzer::{PacketAnalyzer, PacketInfo};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyModule};
use std::error::Error;

pub struct PythonEngine {
    module: Py<PyModule>,
    analyze: Py<PyAny>,
}

impl PythonEngine {
    pub fn new(script_path: &str) -> Result<Self, Box<dyn Error>> {
        Python::with_gil(|py| {
            let code = std::fs::read_to_string(script_path)?;
            let module = PyModule::from_code(py, &code, script_path, "analyze")?;
            let analyze = module.getattr("analyze_packet")?.into_py(py);
            Ok(Self {
                module: module.into_py(py),
               analyze,
            })
        })
    }
}

impl PacketAnalyzer for PythonEngine {
    fn analyze(&self, pkt: &PacketInfo) -> Result<(), Box<dyn Error>> {
        Python::with_gil(|py| {
            let d = PyDict::new(py);
            d.set_item("eth_src",  &pkt.eth_src)?;
            d.set_item("eth_dst",  &pkt.eth_dst)?;
            d.set_item("eth_type", &pkt.eth_type)?;
            d.set_item("src_ip",   &pkt.src_ip)?;
            d.set_item("dst_ip",   &pkt.dst_ip)?;
            d.set_item("ttl",      pkt.ttl)?;
            d.set_item("length",   pkt.length)?;
            d.set_item("src_port", pkt.src_port)?;
            d.set_item("dst_port", pkt.dst_port)?;
            d.set_item("protocol", &pkt.protocol)?;
            d.set_item("icmp_type", pkt.icmp_type)?;
            d.set_item("icmp_code", pkt.icmp_code)?;
            d.set_item("arp_op",         &pkt.arp_op)?;
            d.set_item("arp_sender_ip",  &pkt.arp_sender_ip)?;
            d.set_item("arp_target_ip",  &pkt.arp_target_ip)?;
            d.set_item("arp_sender_mac", &pkt.arp_sender_mac)?;
            d.set_item("arp_target_mac", &pkt.arp_target_mac)?;
            d.set_item("payload", PyList::new(py, &pkt.payload))?;

            self.analyze.call1(py, (d,))?;
            Ok(())
        })
    }
}
