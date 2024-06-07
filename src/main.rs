use std::fs::File;
use std::io::Read;

mod Protocol;

use Protocol::BytePacketBuffer;
use Protocol::DNSPacket;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("./response_packet.txt")?;
    let mut buffer = BytePacketBuffer::BytePacketBuffer::new();
    f.read(&mut buffer.buf)?;

    let packet = DNSPacket::DnsPacket::from_buffer(&mut buffer)?;
    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}