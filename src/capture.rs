use crate::analyzer::{PacketAnalyzer, PacketInfo};
use pcap::Capture;
use pnet::packet::arp::ArpPacket;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::icmpv6::Icmpv6Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::error::Error;

pub fn run<A: PacketAnalyzer>(analyzer: &A) -> Result<(), Box<dyn Error>> {
    let devices = pcap::Device::list()?;
    println!("Available devices:");
    for (i, d) in devices.iter().enumerate() {
        println!("  {}: {} - {}", i, d.name, d.desc.as_deref().unwrap_or("n/a"));
    }
    let dev = devices.into_iter().next().ok_or("no devices")?;
    println!("Capturing on {} (promisc)", dev.name);

    let mut cap = Capture::from_device(dev)?
    .promisc(true)
    .snaplen(65535)
    .timeout(500)
    .open()?;

    cap.filter("", true).ok();

    loop {
        match cap.next_packet() {
            Ok(p) => {
                if let Some(info) = parse_ethernet(p.data) {
                    if let Err(e) = analyzer.analyze(&info) {
                        eprintln!("analyzer error: {e}");
                    }
                }
            }
            Err(pcap::Error::TimeoutExpired) => continue,
            Err(e) => {
                eprintln!("capture error: {e}");
                break;
            }
        }
    }
    Ok(())
}

fn parse_ethernet(data: &[u8]) -> Option<PacketInfo> {
    let eth = EthernetPacket::new(data)?;
    let mut info = PacketInfo {
        eth_src: eth.get_source().to_string(),
        eth_dst: eth.get_destination().to_string(),
        eth_type: format!("{:?}", eth.get_ethertype()),
        length: data.len() as u32,
        ..Default::default()
    };

    match eth.get_ethertype() {
        EtherTypes::Ipv4 => parse_ipv4(eth.payload(), &mut info),
        EtherTypes::Ipv6 => parse_ipv6(eth.payload(), &mut info),
        EtherTypes::Arp  => parse_arp(eth.payload(), &mut info),
        _ => {
            info.protocol = "Other".into();
            info.payload = eth.payload().to_vec();
        }
    }
    Some(info)
}

fn parse_ipv4(bytes: &[u8], info: &mut PacketInfo) {
    let Some(ip) = Ipv4Packet::new(bytes) else { return };
    info.src_ip = Some(ip.get_source().to_string());
    info.dst_ip = Some(ip.get_destination().to_string());
    info.ttl = Some(ip.get_ttl());
    info.protocol = format!("{:?}", ip.get_next_level_protocol());

    match ip.get_next_level_protocol() {
        IpNextHeaderProtocols::Tcp => parse_tcp(ip.payload(), info),
        IpNextHeaderProtocols::Udp => parse_udp(ip.payload(), info),
        IpNextHeaderProtocols::Icmp => parse_icmp(ip.payload(), info),
        _ => info.payload = ip.payload().to_vec(),
    }
}

fn parse_ipv6(bytes: &[u8], info: &mut PacketInfo) {
    let Some(ip) = Ipv6Packet::new(bytes) else { return };
    info.src_ip = Some(ip.get_source().to_string());
    info.dst_ip = Some(ip.get_destination().to_string());
    info.protocol = format!("{:?}", ip.get_next_header());

    match ip.get_next_header() {
        IpNextHeaderProtocols::Tcp => parse_tcp(ip.payload(), info),
        IpNextHeaderProtocols::Udp => parse_udp(ip.payload(), info),
        IpNextHeaderProtocols::Icmpv6 => parse_icmpv6(ip.payload(), info),
        _ => info.payload = ip.payload().to_vec(),
    }
}

fn parse_tcp(bytes: &[u8], info: &mut PacketInfo) {
    if let Some(tcp) = TcpPacket::new(bytes) {
        info.src_port = Some(tcp.get_source());
        info.dst_port = Some(tcp.get_destination());
        info.payload = tcp.payload().to_vec();
        info.protocol = "TCP".into();
    }
}

fn parse_udp(bytes: &[u8], info: &mut PacketInfo) {
    if let Some(udp) = UdpPacket::new(bytes) {
        info.src_port = Some(udp.get_source());
        info.dst_port = Some(udp.get_destination());
        info.payload = udp.payload().to_vec();
        info.protocol = "UDP".into();
    }
}

fn parse_icmp(bytes: &[u8], info: &mut PacketInfo) {
    if let Some(icmp) = IcmpPacket::new(bytes) {
        info.icmp_type = Some(icmp.get_icmp_type().0);
        info.icmp_code = Some(icmp.get_icmp_code().0);
        info.protocol = "ICMP".into();
        info.payload = icmp.payload().to_vec();
    }
}

fn parse_icmpv6(bytes: &[u8], info: &mut PacketInfo) {
    if let Some(icmp) = Icmpv6Packet::new(bytes) {
        info.icmp_type = Some(icmp.get_icmpv6_type().0);
        info.icmp_code = Some(icmp.get_icmpv6_code().0);
        info.protocol = "ICMPv6".into();
        info.payload = icmp.payload().to_vec();
    }
}

fn parse_arp(bytes: &[u8], info: &mut PacketInfo) {
    if let Some(arp) = ArpPacket::new(bytes) {
        info.arp_op = Some(format!("{:?}", arp.get_operation()));
        info.arp_sender_ip = Some(arp.get_sender_proto_addr().to_string());
        info.arp_target_ip = Some(arp.get_target_proto_addr().to_string());
        info.arp_sender_mac = Some(arp.get_sender_hw_addr().to_string());
        info.arp_target_mac = Some(arp.get_target_hw_addr().to_string());
        info.protocol = "ARP".into();
        info.payload = arp.payload().to_vec();
    }
}
