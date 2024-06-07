# DNS_Server-Rust
Building a DNS Server from Scratch in RUST! I have written notes about all the things I have learned in this Readme file, they are written in Hinglish for my referrence.


## Part 1 : Implementing the protocol

DNS Packets ko bhejte h over UDP transport and limited to 512 bytes(Wo alag baat h there is exception where it can be sent over TCP as well and eDNS se packet size badha sakte h).

DNS uses the same format in queries and responses. Mainly a DNS packet consists of:
- Header : Isme hoga information about the query/response
- Question Section : List of questions(hota ek hi h in practice) , each indicating the query name(domain) and the record type of interest
- Answer Section : List of relevant records of the requested type
- Authority Section : A list of name servers used for resolving queries recursively.
- Additional Section : Additional useful info.

Essentially 3 different objects ko support karna hoga:
- Header : [dnsheader.rs](src/protocol/dnsheader.rs) mein implement kar diye : Iske liye we created another implementaion for the rescode field also in [resultcode.rs](src/protocol/resultcode.rs). RCode is set by the server to indicate the status of the response, i.e. whether or not it was successful or failed, and agar fail hua toh providing details about the cause of the failure.
- Question : [dnsquestion.rs](src/protocol/dnsquestion.rs) : Iske liye we created another implementation of [querytype](src/protocol/querytype.rs), so that we can represent the *record type* being queried. 
- Record : [dnsrecord.rs](src/protocol/dnsrecord.rs) is used to represent the actual dns records and allow us to add new records later on easily.


[byte_packet_buffer.rs](src/protocol/byte_packet_buffer.rs) asli problematic kaam karta h. The thing is DNS encodes each name into a sequence of labels, with each label prepended by a single byte indicating its length. Example would be *[3]www[6]google[3]com[0]*. Ye phir bhi theek h, but it gets even more problematic when jumps come into place. 

> Due to the original size constraints of DNS, of 512 bytes for a single packet, some type of compression was needed. Since most of the space required is for the domain names, and part of the same name tends to reoccur, there's some obvious space saving opportunity.

To save space from reoccuring set of characters, DNS packets include a "jump directive", telling the packet parser to jump to another position, and finish reading the name there. This _jump_ can be read if the length byte has the two most significant bits set,iska matlab jump hai, and we need to follow the pointer.

Ek aur baat to be taken care of is, this jumps can cause a cycle if some problematic person adds it to the packet, so wo check karna padega. This along with reading of the packets is all implemented in the byte_packet_buffer.

Bas phir, we can put together all of this now in our [dnspacket.rs](src/protocol/dnspacket.rs) to finish our protocol implementation.

To test it out, run the [main.rs](src/protocol/main.rs) file with our `response_packet.txt` 

## Part 2 : Building a stub resolver

A stub resolver is a DNS Client that doesn't feature any built-in support for recursive lookup and that will only work with a DNS server that does.

- We need to extend our [byte_packet_buffer.rs](src/protocol/byte_packet_buffer.rs) to add methods for writing bytes and for writing query names in labeled form. Additionally, we will be extending our Header, Record, Question and Packet structs.

- Next we can implement a Stub Resolver using the *UDPSocket* included in rust, instead of having to read a packet file.

The output of running the stub resolver was : 
```
cargo run
warning: crate `DNS_Server_Rust` should have a snake case name
  |
  = help: convert the identifier to snake case: `dns_server_rust`
  = note: `#[warn(non_snake_case)]` on by default

warning: `DNS-Server-Rust` (bin "DNS-Server-Rust") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/DNS-Server-Rust`
DnsHeader {
    id: 6666,
    recursion_desired: true,
    truncated_message: false,
    authoritative_answer: false,
    opcode: 0,
    response: true,
    rescode: NOERROR,
    checking_disabled: false,
    authed_data: false,
    z: false,
    recursion_available: true,
    questions: 1,
    answers: 1,
    authoritative_entries: 0,
    resource_entries: 0,
}
DnsQuestion {
    name: "google.com",
    qtype: A,
}
A {
    domain: "google.com",
    addr: 172.217.167.142,
    ttl: 80,
}
```
