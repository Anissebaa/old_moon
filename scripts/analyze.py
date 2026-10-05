import ipaddress

FILTERS = {
    "src_subnet": "192.168.1.0/24",
    "protocols":  None,
    "ports":      None,
    "show_arp":   True,
}


def _in_cidr(ip, cidr):
    if not ip:
        return False
    try:
        return ipaddress.ip_address(ip) in ipaddress.ip_network(cidr, strict=False)
    except ValueError:
        return ip == cidr


def _should_print(p):
    if FILTERS["protocols"] and p["protocol"] not in FILTERS["protocols"]:
        return False
    if not FILTERS["show_arp"] and p["protocol"] == "ARP":
        return False
    if FILTERS["ports"]:
        if p["dst_port"] not in FILTERS["ports"] and p["src_port"] not in FILTERS["ports"]:
            return False
    if p["src_ip"] or p["dst_ip"]:
        return _in_cidr(p["src_ip"], FILTERS["src_subnet"]) or \
               _in_cidr(p["dst_ip"], FILTERS["src_subnet"])
    return True


def analyze_packet(p):
    if not _should_print(p):
        return

    if p["protocol"] == "ARP":
        print(f"[ARP] {p['arp_sender_ip']} ({p['arp_sender_mac']}) -> "
              f"{p['arp_target_ip']} ({p['arp_target_mac']}) op={p['arp_op']}")
        return

    if p["protocol"] in ("ICMP", "ICMPv6"):
        print(f"[{p['protocol']}] {p['src_ip']} -> {p['dst_ip']} "
              f"type={p['icmp_type']} code={p['icmp_code']} len={p['length']}")
        return

    print(f"[{p['protocol']}] {p['src_ip']}:{p['src_port']} -> "
          f"{p['dst_ip']}:{p['dst_port']} ttl={p['ttl']} "
          f"len={p['length']} payload={len(p['payload'])}")

    if p["protocol"] == "TCP" and p["dst_port"] == 80 and p["payload"]:
        hexs = " ".join(f"{b:02x}" for b in p["payload"][:32])
        print("    HTTP:", hexs)
