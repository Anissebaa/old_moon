local function ip_to_int(ip)
local a, b, c, d = ip:match("^(%d+)%.(%d+)%.(%d+)%.(%d+)$")
if not a then return nil end
    return (tonumber(a) << 24) | (tonumber(b) << 16) | (tonumber(c) << 8) | tonumber(d)
    end

    local function in_cidr(ip, cidr)
    if not ip then return false end
        local net, bits = cidr:match("^([%d%.]+)/(%d+)$")
        if not net then return ip == cidr end
            bits = tonumber(bits)
            local mask = bits == 0 and 0 or ((0xFFFFFFFF << (32 - bits)) & 0xFFFFFFFF)
            local ip_i  = ip_to_int(ip)
            local net_i = ip_to_int(net)
            if not ip_i or not net_i then return false end
                return (ip_i & mask) == (net_i & mask)
                end

                local function hexdump(s, n)
                n = math.min(n or 32, #s)
                local out = {}
                for i = 1, n do out[#out+1] = string.format("%02x", s:byte(i)) end
                    return table.concat(out, " ")
                    end

                    local FILTERS = {
                        src_subnet = "192.168.1.0/24",
                        protocols  = nil,
                        ports      = nil,
                        show_arp   = true,
                    }

                    local function should_print(p)
                    if FILTERS.protocols and not FILTERS.protocols[p.protocol] then return false end
                        if FILTERS.show_arp == false and p.protocol == "ARP" then return false end
                            if FILTERS.ports and p.dst_port and not FILTERS.ports[p.dst_port] then
                                if not (p.src_port and FILTERS.ports[p.src_port]) then return false end
                                    end
                                    if p.src_ip or p.dst_ip then
                                        return in_cidr(p.src_ip, FILTERS.src_subnet)
                                        or in_cidr(p.dst_ip, FILTERS.src_subnet)
                                        end
                                        return true
                                        end

                                        function analyze_packet(p)
                                        if not should_print(p) then return end

                                            if p.protocol == "ARP" then
                                                print(string.format(
                                                    "[ARP] %s (%s) -> %s (%s) op=%s",
                                                                    p.arp_sender_ip or "?", p.arp_sender_mac or "?",
                                                                    p.arp_target_ip or "?", p.arp_target_mac or "?",
                                                                    p.arp_op or "?"))
                                                return
                                                end

                                                if p.protocol == "ICMP" or p.protocol == "ICMPv6" then
                                                    print(string.format(
                                                        "[%s] %s -> %s  type=%s code=%s  len=%d",
                                                        p.protocol, p.src_ip or "?", p.dst_ip or "?",
                                                        tostring(p.icmp_type), tostring(p.icmp_code), p.length))
                                                    return
                                                    end

                                                    print(string.format(
                                                        "[%s] %s:%s -> %s:%s  ttl=%s len=%d payload=%d",
                                                        p.protocol,
                                                        p.src_ip or "?", p.src_port and tostring(p.src_port) or "-",
                                                                        p.dst_ip or "?", p.dst_port and tostring(p.dst_port) or "-",
                                                                        p.ttl and tostring(p.ttl) or "-",
                                                                        p.length, #p.payload))

                                                    if p.protocol == "TCP" and p.dst_port == 80 and #p.payload > 0 then
                                                        print("    HTTP: " .. hexdump(p.payload, 32))
                                                        end
                                                        end
