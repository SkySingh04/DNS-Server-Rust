use std::fs::File;
// use std::io::Read;
use std::net::UdpSocket;

mod protocol;

use protocol::byte_packet_buffer::BytePacketBuffer;
use protocol::dnspacket::DnsPacket;
use protocol::querytype::QueryType;
use protocol::dnsquestion::DnsQuestion;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Perform an A query for www.google.com
    let qname = "google.com";
    let qtype = QueryType::A;

    //Using google's public DNS Server
    let server = ("8.8.8.8" , 53);

    //Bind a UDP socket to an arbitrary port
    let socket = UdpSocket::bind(("0.0.0.0" , 43210))?; //Bind to port 43210

    //Build our query packet. It's important that we remember to set the 
    //`recursion_desired` flag to true. As noted earlier, the packet id is arbitraty.

    let mut packet = DnsPacket::new();

    packet.header.id = 6666;
    packet.header.questions =1;
    packet.header.recursion_desired = true;

    packet.questions.push(DnsQuestion::new(qname.to_string(), qtype));

    //Write the packet to a buffer
    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer)?;

    //Send it to the server using our socket
    socket.send_to(&req_buffer.buf[0..req_buffer.pos], server)?;

    //To prepare for receiving the response, we'll create a new `BytePacketBuffer`
    //and ask the socket to write the response directly into our buffer.
    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf)?;

    //We can now use the `from_buffer` method to parse the response into a `DnsPacket` and then print the response
    let res_packet = DnsPacket::from_buffer(&mut res_buffer)?;
    println!("{:#?}", res_packet.header);

    for q in res_packet.questions {
        println!("{:#?}", q);
    }
    for rec in res_packet.answers {
        println!("{:#?}", rec);
    }
    for rec in res_packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in res_packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}