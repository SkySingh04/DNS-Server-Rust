use crate::protocol::byte_packet_buffer::BytePacketBuffer;
use crate::protocol::querytype::QueryType;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum DnsRecord {
    Unknown {
        domain: String,
        qtype: u16,
        data_len: u16,
        ttl: u32,
    }, // 0
    A {
        domain: String,
        addr: Ipv4Addr,
        ttl: u32,
    }, // 1
    NS {
        domain: String,
        host: String,
        ttl: u32,
    }, // 2
    Cname {
        domain: String,
        host: String,
        ttl: u32,
    }, // 5
    Soa {
        domain: String,
        mname: String,
        rname: String,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32,
        ttl: u32,
    }, // 6
    MX {
        domain: String,
        priority: u16,
        host: String,
        ttl: u32,
    }, // 15
    Txt {
        domain: String,
        text: String,
        ttl: u32,
    }, // 16
    Aaaa {
        domain: String,
        addr: Ipv6Addr,
        ttl: u32,
    }, // 28
}

impl DnsRecord {
    pub fn read(buffer: &mut BytePacketBuffer) -> Result<DnsRecord, Box<dyn std::error::Error>> {
        let mut domain = String::new();
        buffer.read_qname(&mut domain)?;

        let qtype_num = buffer.read_u16()?;
        let qtype = QueryType::from_num(qtype_num);
        let _ = buffer.read_u16()?;
        let ttl = buffer.read_u32()?;
        let data_len = buffer.read_u16()?;

        match qtype {
            QueryType::A => {
                let raw_addr = buffer.read_u32()?;
                let addr = Ipv4Addr::new(
                    ((raw_addr >> 24) & 0xFF) as u8,
                    ((raw_addr >> 16) & 0xFF) as u8,
                    ((raw_addr >> 8) & 0xFF) as u8,
                    ((raw_addr) & 0xFF) as u8,
                );
        
                Ok(DnsRecord::A {
                    domain,
                    addr,
                    ttl,
                })
            }

            QueryType::Aaaa => {
                let raw_addr1 = buffer.read_u32()?;
                let raw_addr2 = buffer.read_u32()?;
                let raw_addr3 = buffer.read_u32()?;
                let raw_addr4 = buffer.read_u32()?;
                let addr = Ipv6Addr::new(
                    ((raw_addr1 >> 16) & 0xFFFF) as u16,
                    (raw_addr1 & 0xFFFF) as u16,
                    ((raw_addr2 >> 16) & 0xFFFF) as u16,
                    ((raw_addr2) & 0xFFFF) as u16,
                    ((raw_addr3 >> 16) & 0xFFFF) as u16,
                    ((raw_addr3) & 0xFFFF) as u16,
                    ((raw_addr4 >> 16) & 0xFFFF) as u16,
                    ((raw_addr4) & 0xFFFF) as u16,
                );

                Ok(DnsRecord::Aaaa {
                    domain,
                    addr,
                    ttl,
                })
            }

            QueryType::NS => {
                let mut ns = String::new();
                buffer.read_qname(&mut ns)?;
        
                Ok(DnsRecord::NS {
                    domain,
                    host: ns,
                    ttl,
                })
            }

            QueryType::Cname => {
                let mut cname = String::new();
                buffer.read_qname(&mut cname)?;
        
                Ok(DnsRecord::Cname {
                    domain,
                    host: cname,
                    ttl,
                })
            }

            QueryType::Soa => {
                let mut mname = String::new();
                buffer.read_qname(&mut mname)?;
        
                let mut rname = String::new();
                buffer.read_qname(&mut rname)?;
        
                let serial = buffer.read_u32()?;
                let refresh = buffer.read_u32()?;
                let retry = buffer.read_u32()?;
                let expire = buffer.read_u32()?;
                let minimum = buffer.read_u32()?;
        
                Ok(DnsRecord::Soa {
                    domain,
                    mname,
                    rname,
                    serial,
                    refresh,
                    retry,
                    expire,
                    minimum,
                    ttl,
                })
            }

            QueryType::MX => {
                let priority = buffer.read_u16()?;
        
                let mut mx = String::new();
                buffer.read_qname(&mut mx)?;
        
                Ok(DnsRecord::MX {
                    domain,
                    priority,
                    host: mx,
                    ttl,
                })
            }

            QueryType::Txt => {
                let mut txt = String::new();
                buffer.read_qname(&mut txt)?;
        
                Ok(DnsRecord::Txt {
                    domain,
                    text: txt,
                    ttl,
                })
            }

            QueryType::Unknown(_) => {
                buffer.step(data_len as usize)?;
        
                Ok(DnsRecord::Unknown {
                    domain,
                    qtype: qtype_num,
                    data_len,
                    ttl,
                })
            }
        }
    }
    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<usize, Box<dyn std::error::Error>> {
        let start_pos = buffer.pos();

        match *self {
            DnsRecord::A {
                ref domain,
                ref addr,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::A.to_num())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;
                buffer.write_u16(4)?;

                let octets = addr.octets();
                buffer.write_u8(octets[0])?;
                buffer.write_u8(octets[1])?;
                buffer.write_u8(octets[2])?;
                buffer.write_u8(octets[3])?;
            }
            DnsRecord::Unknown { .. } => {
                println!("Skipping record: {:?}", self);
            }
            DnsRecord::NS {
                ref domain,
                ref host,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::NS.to_num())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;

                let pos = buffer.pos();
                buffer.write_u16(0)?;

                buffer.write_qname(host)?;

                let size = buffer.pos() - (pos + 2);
                buffer.set_u16(pos, size as u16)?;
            }
            DnsRecord::Cname {
                ref domain,
                ref host,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::Cname.to_num())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;

                let pos = buffer.pos();
                buffer.write_u16(0)?;

                buffer.write_qname(host)?;

                let size = buffer.pos() - (pos + 2);
                buffer.set_u16(pos, size as u16)?;
            }
            DnsRecord::Soa {
                ref domain,
                ref mname,
                ref rname,
                serial,
                refresh,
                retry,
                expire,
                minimum,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::Soa.to_num())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;

                let pos = buffer.pos();
                buffer.write_u16(0)?;

                buffer.write_qname(mname)?;
                buffer.write_qname(rname)?;
                buffer.write_u32(serial)?;
                buffer.write_u32(refresh)?;
                buffer.write_u32(retry)?;
                buffer.write_u32(expire)?;
                buffer.write_u32(minimum)?;

                let size = buffer.pos() - (pos + 2);
                buffer.set_u16(pos, size as u16)?;
            }
            DnsRecord::MX {
                ref domain,
                priority,
                ref host,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::MX.to_num())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;

                let pos = buffer.pos();
                buffer.write_u16(0)?;

                buffer.write_u16(priority)?;
                buffer.write_qname(host)?;

                let size = buffer.pos() - (pos + 2);
                buffer.set_u16(pos, size as u16)?;
            }
            DnsRecord::Txt {
                ref domain,
                ref text,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::Txt.to_num())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;

                let pos = buffer.pos();
                buffer.write_u16(0)?;

                buffer.write_qname(text)?;

                let size = buffer.pos() - (pos + 2);
                buffer.set_u16(pos, size as u16)?;
            }
            DnsRecord::Aaaa {
                ref domain,
                ref addr,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::Aaaa.to_num())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;
                buffer.write_u16(16)?;

                let segments = addr.segments();
                buffer.write_u16(segments[0])?;
                buffer.write_u16(segments[1])?;
                buffer.write_u16(segments[2])?;
                buffer.write_u16(segments[3])?;
                buffer.write_u16(segments[4])?;
                buffer.write_u16(segments[5])?;
                buffer.write_u16(segments[6])?;
                buffer.write_u16(segments[7])?;
            }
        }
        Ok(buffer.pos() - start_pos)
    }
}