use std::error::Error;
// use crate::File;
use std::fs::File;
use std::io::Read;
use crate::protocol::byte_packet_buffer::BytePacketBuffer;
use crate::protocol::dnspacket::DnsPacket;

#[allow(dead_code)]
fn test_with_response_packet() -> Result<(), Box<dyn Error>> {
    let mut f = File::open("response_packet.Txt")?;
    let mut buffer = BytePacketBuffer::new();
    let _ = f.read(&mut buffer.buf)?;

    let packet = DnsPacket::from_buffer(&mut buffer)?;
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
