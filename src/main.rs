// use std::fs::File;
// use std::io::Read;
use std::net::UdpSocket;
// use std::result;

mod protocol;

use protocol::byte_packet_buffer::BytePacketBuffer;
use protocol::dnspacket::DnsPacket;
use protocol::querytype::QueryType;
use protocol::dnsquestion::DnsQuestion;
use protocol::resultcode::ResultCode;
use std::net::Ipv4Addr;

fn lookup(qname : &str , qtype: QueryType , server : (Ipv4Addr , u16)) -> Result<DnsPacket ,  Box<dyn std::error::Error>> {
    //Forward queries to Google's public DNS server
    // let server = ("8.8.8.8",53);

    let socket = UdpSocket::bind(("0.0.0.0" , 43210))?;

    let mut packet = DnsPacket::new();

    packet.header.id = 6666;
    packet.header.questions =1;
    packet.header.recursion_desired = true;

    packet.questions.push(DnsQuestion::new(qname.to_string(), qtype));

    let mut req_buffer = BytePacketBuffer::new();
    let _ = packet.write(&mut req_buffer);
    socket.send_to(&req_buffer.buf[0..req_buffer.pos], server)?;

    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf)?;

    DnsPacket::from_buffer(&mut res_buffer)
}

//Handle a single incoming packet with this
fn handle_query(socket : &UdpSocket) -> Result<() , Box<dyn std::error::Error>> {
    //With a socket ready, we can go ahead and read a packet.
    //This will block until one is received

    let mut req_buffer = BytePacketBuffer::new();

    //The `recv_from` function will write the data into the provided buffer,
    //And return the length of the data read as well as the source address.
    //We are essentially not interested in length , but we need to keep track of
    //the source in order to send our reply later on.

    let (_ , src) = socket.recv_from(&mut req_buffer.buf)?;

    //Next, we'll parse the packet into a `DnsPacket` struct
    let mut request = DnsPacket::from_buffer(&mut req_buffer)?;

    //We'll create a new packet to hold our response
    let mut packet  = DnsPacket::new();
    
    packet.header.id = request.header.id;
    packet.header.response = true;
    packet.header.recursion_available = true;
    packet.header.recursion_desired = true;

    //In the normal case, exactly one question is present
    if let Some(question) = request.questions.pop() {
        println!("Received query: {:?}", question);

        //Since all is set up and as expectrd, the query can be forwarded to the 
        //target server. There's always the possibility that the query will
        //fail, in which case the `SERVFAIL` response code is set to indicate as much to the client.
        //If rather everything goes and planned, the question and response records as copied into our response packet.

        // if let Ok(result) = lookup(&question.name, question.qtype) {
        //     packet.questions.push(question);
        //     packet.header.rescode = result.header.rescode;
        if let Ok(result) = recursive_lookup(&question.name, question.qtype) {
            packet.questions.push(question.clone());
            packet.header.rescode = result.header.rescode;


            for rec in result.answers {
                println!("Answer: {:?}", rec);
                packet.answers.push(rec);
            }

            for rec in result.authorities {
                println!("Authority: {:?}", rec);
                packet.authorities.push(rec);
            }

            for rec in result.resources {
                println!("Resource: {:?}", rec);
                packet.resources.push(rec);
            }
        }
        else{
            packet.header.rescode =  ResultCode::ServFail;
        }
    }
    //We need to make sure that a question is actually present in the packet
    //If not , we'll set the response code to `FORMERR` and return an error
    else{
        packet.header.rescode = ResultCode::FormErr;
    }

    //Now we can just encode our response and send it back to the client
    let mut res_buffer = BytePacketBuffer::new();

    let _ = packet.write(&mut res_buffer);

    let len = res_buffer.pos();
    let data = res_buffer.get_range(0, len)?;

    socket.send_to(data, src)?;

    Ok(())
}

//Implementing recursive lookup
pub fn recursive_lookup(qname : &str, qtype : QueryType) -> Result <DnsPacket , Box<dyn std::error::Error>> {
    //for now we're always starting with *a.root-servers.net*
    let mut ns = "198.41.0.4".parse::<Ipv4Addr>().unwrap();

    //Since it might take an arbitrary number of queries to get to the final answer,
    //We start the loop
    loop {
        println!("Attempting lookup of {:?} {} with ns {}", qtype, qname, ns);

        //The next step is to send the query to the active server
        let ns_copy = ns;

        let server = (ns_copy, 53);
        let response = lookup(qname , qtype , server)?;

        //If there are entries in the answer section, we can return the packet
        if !response.answers.is_empty() && response.header.rescode == ResultCode::NoError {
            return Ok(response);
        }

        // We might also get a `NXDOMAIN` reply, which is the authoritative name servers
        // way of telling us that the name doesn't exist.
        if response.header.rescode == ResultCode::NXDomain {
            return Ok(response);
        }

        // Otherwise, we'll try to find a new nameserver based on NS and a corresponding A
        // record in the additional section. If this succeeds, we can switch name server
        // and retry the loop.
        if let Some(new_ns) = response.get_resolved_ns(qname) {
            ns = new_ns;

            continue;
        }
        // If not, we'll have to resolve the ip of a NS record. If no NS records exist,
    // we'll go with what the last server told us.
    let new_ns_name = match response.get_unresolved_ns(qname) {
        Some(x) => x,
        None => return Ok(response),
    };

    // Here we go down the rabbit hole by starting _another_ lookup sequence in the
    // midst of our current one. Hopefully, this will give us the IP of an appropriate
    // name server.
    let recursive_response = recursive_lookup(new_ns_name, QueryType::A)?;

    // Finally, we pick a random ip from the result, and restart the loop. If no such
    // record is available, we again return the last result we got.
    if let Some(new_ns) = recursive_response.get_random_a() {
        ns = new_ns;
    } else {
        return Ok(response);
    }
}
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
 //Bind an UDP socket on port 2053
 let socket = UdpSocket::bind(("0.0.0.0" , 2053))?;

 println!("Server started successfully on port 2053");
 //For now, queries area handled sequentially, so an infinite loop for requests is initiated
 loop {
    match handle_query(&socket) {
        Ok(_) =>{},
        Err(e) =>eprintln!("An error occured : {}",e),
    }
 }
}