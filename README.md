# A DNS Server : Built with RUST!
Building a DNS Server from Scratch in RUST!(yes I consider this language to be the language of the cool kids and yes I struggle to write a simple function in it and yes I want to earn money without having to do full stack). 

I have written notes about all the things I have learned in this Readme file, they are written in Hinglish for my reference.


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

The output will be : 
```
cargo run
   Compiling DNS-Server-Rust v0.1.0 (/home/akash/Desktop/rust/DNS-Server-Rust)

warning: `DNS-Server-Rust` (bin "DNS-Server-Rust") generated 8 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/DNS-Server-Rust`
DnsHeader {
    id: 16488,
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
    addr: 142.250.183.238,
    ttl: 74,
}
```

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

## Part 3 : Adding More Record types

Currently we support only A type records. But ofcourse, there are n number of record types (most of them don't see any use) but some important ones are: 

| ID  | Name  | Description                                              | Encoding                                         |
| --- | ----- | -------------------------------------------------------- | ------------------------------------------------ |
| 1   | A     | Alias - Mapping names to IP addresses                    | Preamble + Four bytes for IPv4 adress            |
| 2   | NS    | Name Server - The DNS server address for a domain        | Preamble + Label Sequence                        |
| 5   | CNAME | Canonical Name - Maps names to names                     | Preamble + Label Sequence                        |
| 6   | SOA   | Start of Authority - Provides authoritative information  | Preamble + Label Sequence          |
| 15  | MX    | Mail eXchange - The host of the mail server for a domain | Preamble + 2-bytes for priority + Label Sequence |
| 16  | TXT   | Text - Arbitrary text data associated with a domain      | Preamble + Text data                              |
| 28  | AAAA  | IPv6 address                                             | Preamble + Sixteen bytes for IPv6 address        |

- We need to update our `QueryType` enum and change our utility functions. We also need to extend our `DnsRecord` for reading new record types and extend the functions foe reading and writing the new type of records.

Now if we query for *Yahoo.com* with QueryType set as *MX*, we can see our new type of records:

```
cargo run
   Compiling DNS-Server-Rust v0.1.0 (/home/akash/Desktop/rust/DNS-Server-Rust)

warning: `DNS-Server-Rust` (bin "DNS-Server-Rust") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
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
    answers: 3,
    authoritative_entries: 0,
    resource_entries: 0,
}
DnsQuestion {
    name: "yahoo.com",
    qtype: MX,
}
MX {
    domain: "yahoo.com",
    priority: 1,
    host: "mta5.am0.yahoodns.net",
    ttl: 1673,
}
MX {
    domain: "yahoo.com",
    priority: 1,
    host: "mta7.am0.yahoodns.net",
    ttl: 1673,
}
MX {
    domain: "yahoo.com",
    priority: 1,
    host: "mta6.am0.yahoodns.net",
    ttl: 1673,
}
```

And for *meetakash.vercel.app* with *SOA* query type:
```
cargo run
   Compiling DNS-Server-Rust v0.1.0 (/home/akash/Desktop/rust/DNS-Server-Rust)

warning: `DNS-Server-Rust` (bin "DNS-Server-Rust") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
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
    answers: 0,
    authoritative_entries: 1,
    resource_entries: 0,
}
DnsQuestion {
    name: "meetakash.vercel.app",
    qtype: SOA,
}
SOA {
    domain: "vercel.app",
    mname: "ns1.vercel-dns.com",
    rname: "hostmaster.nsone.net",
    serial: 1659373707,
    refresh: 43200,
    retry: 7200,
    expire: 1209600,
    minimum: 14400,
    ttl: 1800,
}
```

## Part 4 : Ab banega Actual DNS Server

There are essentially two types of DNS servers, A DNS server can do both in theory but usually they are mutually exclusive.

- *Authoritative Server* : A DNS Server hosting one or more "zones".ex: The authoritative servers for the zone google.com are ns1.google.com, ns2.google.com, ns3.google.com and ns4.google.com.

- *Caching Server* : A DNS server that serves DNS lookups by first checking its chache to see if it already knows of rhe record being requested, and if not performs a recursive lookup.

First we can implement a server that simply forwards queries to another caching server, i.e. a "DNS proxy server". We will refactor our [main.rs](src/main.rs) by moving our lookup code into a separate function. Along with that, we will write our server code to handle requests.

Now we can start our server in one terminal and then use `dig` to perform lookup in a second terminal.

```
dig @127.0.0.1 -p 2053 meetakash.vercel.app

; <<>> DiG 9.18.18-0ubuntu0.22.04.2-Ubuntu <<>> @127.0.0.1 -p 2053 meetakash.vercel.app
; (1 server found)
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 37880
;; flags: qr rd ra; QUERY: 1, ANSWER: 2, AUTHORITY: 0, ADDITIONAL: 0

;; QUESTION SECTION:
;meetakash.vercel.app.          IN      A

;; ANSWER SECTION:
meetakash.vercel.app.   1800    IN      A       76.76.21.93
meetakash.vercel.app.   1800    IN      A       76.76.21.164

;; Query time: 148 msec
;; SERVER: 127.0.0.1#2053(127.0.0.1) (UDP)
;; WHEN: Sat Jun 08 10:26:37 IST 2024
;; MSG SIZE  rcvd: 110
```

And in our server terminal we can see : 
```
cargo run
   Compiling DNS-Server-Rust v0.1.0 (/home/akash/Desktop/rust/DNS-Server-Rust)
warning: `DNS-Server-Rust` (bin "DNS-Server-Rust") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/DNS-Server-Rust`
Server started successfully on port 2053
Received query: DnsQuestion { name: "google.com", qtype: A }
Answer: A { domain: "google.com", addr: 142.250.196.78, ttl: 245 }
Received query: DnsQuestion { name: "meetakash.vercel.app", qtype: A }
Answer: A { domain: "meetakash.vercel.app", addr: 76.76.21.93, ttl: 1800 }
Answer: A { domain: "meetakash.vercel.app", addr: 76.76.21.164, ttl: 1800 }
```

Lessgooo, we have a DNS server that is able to respond to queries with several different record types!!

## Part 5 : Implementing a recursive resolver

Our server is very nice as it is now, but we are reliant on another server to actually perform the lookup.

The question is first issued to one of the Internet's 13 root servers. Any resolver will need to know of these 13 servers before hand. A file containing all of them, in bind format, is available on the internet and called [named.root](https://www.internic.net/domain/named.root). These servers all contain the same information, and to get started we can pick one of them at random.

The flow is simple:

- The initial query is sent to the root server, which don't know the full `www.google.com` but they do know about `com`, and the reply will tell us where to go next.(This step is usually cached).

- Next again, we will get a list of shortlisted domains that will point us to the domain.

- On this lookup, we will get the IP address of our website.

In practice, a DNS server will maintain a cache, and most TLD's will be known since before. That means that most queries will only ever require two lookups by the server, and commonly one or zero.

Now we can extend [dnspacket.rs](src/protocol/dnspacket.rs) for recursive lookups. Then, we can implement our recursive lookup and change our `handle_query` function to use our new recursive lookup!

Great, now we can see the output as : 
```
dig @127.0.0.1 -p 2053 www.google.com

; <<>> DiG 9.18.18-0ubuntu0.22.04.2-Ubuntu <<>> @127.0.0.1 -p 2053 www.google.com
; (1 server found)
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 35891
;; flags: qr rd ra; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 0

;; QUESTION SECTION:
;www.google.com.                        IN      A

;; ANSWER SECTION:
www.google.com.         300     IN      A       172.217.163.196

;; Query time: 195 msec
;; SERVER: 127.0.0.1#2053(127.0.0.1) (UDP)
;; WHEN: Wed Jun 12 13:42:43 IST 2024
;; MSG SIZE  rcvd: 62

```

And in our server window : 
```
 Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/DNS-Server-Rust`
Server started successfully on port 2053
Received query: DnsQuestion { name: "www.google.com", qtype: A }
Attempting lookup of A www.google.com with ns 198.41.0.4
Attempting lookup of A www.google.com with ns 192.41.162.30
Attempting lookup of A www.google.com with ns 216.239.34.10
Answer: A { domain: "www.google.com", addr: 172.217.163.196, ttl: 300 }
```

And that's it! That is our DNS server completed!

## Contributing

Contributions are welcome! Please read the [CONTRIBUTING.md](CONTRIBUTING.md) file for more details on how to get involved.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.


