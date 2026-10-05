function analyze_packet(pkt)

print(string.format("[%s] %s:%d -> %s:%d | Payload size: %d",
                    pkt.protocol,
                    pkt.src_ip, pkt.src_port,
                    pkt.dst_ip, pkt.dst_port,
                    #pkt.payload))

                    if pkt.protocol == "Tcp" and pkt.dst_port == 80 then
                        local hex = ""
                        for i = 1, math.min(#pkt.payload, 32) do
                            hex = hex .. string.format("%02x ", pkt.payload:byte(i))
                            end
                            print("  HTTP payload (first 32 bytes): " .. hex)
                            end
                            end
