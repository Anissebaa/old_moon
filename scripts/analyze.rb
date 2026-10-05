require 'ipaddr'

FILTERS = {
    src_subnet: '192.168.1.0/24',
    protocols:  nil,
    ports:      nil,
    show_arp:   true
}

def in_cidr?(ip, cidr)
    return false if ip.nil? || ip.empty?
    IPAddr.new(cidr).include?(IPAddr.new(ip))
rescue IPAddr::Error
    ip == cidr
end

def should_print?(p)
    return false if FILTERS[:protocols] && !FILTERS[:protocols].include?(p['protocol'])
    return false if !FILTERS[:show_arp] && p['protocol'] == 'ARP'
    if FILTERS[:ports]
        return false unless FILTERS[:ports].include?(p['dst_port']) ||
                        FILTERS[:ports].include?(p['src_port'])
    end
    if p['src_ip'] || p['dst_ip']
        return in_cidr?(p['src_ip'], FILTERS[:src_subnet]) ||
           in_cidr?(p['dst_ip'], FILTERS[:src_subnet])
    end
    true
end

def analyze_packet(p)
    return unless should_print?(p)

    case p['protocol']
    when 'ARP'
        puts "[ARP] #{p['arp_sender_ip']} (#{p['arp_sender_mac']}) -> " \
         "#{p['arp_target_ip']} (#{p['arp_target_mac']}) op=#{p['arp_op']}"
    when 'ICMP', 'ICMPv6'
        puts "[#{p['protocol']}] #{p['src_ip']} -> #{p['dst_ip']} " \
         "type=#{p['icmp_type']} code=#{p['icmp_code']} len=#{p['length']}"
    else
        puts "[#{p['protocol']}] #{p['src_ip']}:#{p['src_port']} -> " \
         "#{p['dst_ip']}:#{p['dst_port']} ttl=#{p['ttl']} " \
         "len=#{p['length']} payload=#{p['payload'].length}"
        if p['protocol'] == 'TCP' && p['dst_port'] == 80 && p['payload'].any?
            hex = p['payload'].first(32).map { |b| format('%02x', b) }.join(' ')
            puts "    HTTP: #{hex}"
        end
    end
end
