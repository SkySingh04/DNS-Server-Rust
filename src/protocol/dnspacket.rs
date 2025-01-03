use std::net::Ipv4Addr;

use crate::protocol::byte_packet_buffer::BytePacketBuffer;
use crate::protocol::dnsheader::DnsHeader;
use crate::protocol::dnsquestion::DnsQuestion;
use crate::protocol::dnsrecord::DnsRecord;
use crate::protocol::querytype::QueryType;

#[derive(Clone, Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>,
}

impl Default for DnsPacket {
    fn default() -> Self {
        DnsPacket::new()
    }
}

impl DnsPacket {
    pub fn new() -> DnsPacket {
        DnsPacket {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            resources: Vec::new(),
        }
    }

    pub fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<DnsPacket, Box<dyn std::error::Error>> {
        let mut result = DnsPacket::new();
        result.header.read(buffer)?;

        for _ in 0..result.header.questions {
            let mut question = DnsQuestion::new("".to_string(), QueryType::Unknown(0));
            question.read(buffer)?;
            result.questions.push(question);
        }

        for _ in 0..result.header.answers {
            let rec = DnsRecord::read(buffer)?;
            result.answers.push(rec);
        }
        for _ in 0..result.header.authoritative_entries {
            let rec = DnsRecord::read(buffer)?;
            result.authorities.push(rec);
        }
        for _ in 0..result.header.resource_entries {
            let rec = DnsRecord::read(buffer)?;
            result.resources.push(rec);
        }

        Ok(result)
    }
    pub fn write(&mut self, buffer: &mut BytePacketBuffer) -> Result<() ,Box<dyn std::error::Error>> {
        self.header.questions = self.questions.len() as u16;
        self.header.answers = self.answers.len() as u16;
        self.header.authoritative_entries = self.authorities.len() as u16;
        self.header.resource_entries = self.resources.len() as u16;

        self.header.write(buffer)?;

        for question in &self.questions {
            question.write(buffer)?;
        }
        for rec in &self.answers {
            rec.write(buffer)?;
        }
        for rec in &self.authorities {
            rec.write(buffer)?;
        }
        for rec in &self.resources {
            rec.write(buffer)?;
        }

        Ok(())
    }

    //It is useful to be able to pick a random A record from a packet, 
    //since when we get multiple nIP's for a single name, it doesn't matter which one we use.
    //Isliye random A record pick karne ke liye ye function banaya hai.
    pub fn get_random_a(&self) -> Option<Ipv4Addr> {
        self.answers.iter()
        .filter_map(|record| match record {
            DnsRecord::A {addr, ..} => Some(*addr),
            _ => None,
        })
        .next()
    }

    //A helper function which returns an iterator over all name servers
    //in the authorities section, represented as (domain , host) tuples.
    pub fn get_ns<'a>(&'a self , qname : &'a str) -> impl Iterator<Item = (&'a str , &'a str)> {
        self.authorities.iter()
        //In practice, these are always NS records in well formed packages.
        //Convert the NS records to a tuple which has only the data we need
        //to make it easy to work with
        .filter_map(|record| match record {
            DnsRecord::NS { domain, host, .. } => Some((domain.as_str(), host.as_str())),
            _ => None,
        })
        //Filter out the records which are not for the domain we are looking for
        .filter(move |(domain , _)| qname.ends_with(*domain))
    }
    //We will use the fact that name servers often bundle the corresponding A records
    //When repluing to an NS query to implement a function that returns the actual IP 
    //for an NS record if possible.
    pub fn get_resolved_ns (&self , qname : &str) -> Option<Ipv4Addr> {
        //Get an iterator over the nameservers in the authorities section
        self.get_ns(qname)
        //Now we need to look for a matching A record in the additional section.
        //Scince we just want the first one, we can just build a stream of matching records.
        .flat_map(|(_, host)| {
            self.resources.iter()
            // Filter for A records where the domain match the host
            // of the NS record that we are currently processing
            .filter_map(move |record| match record {
                DnsRecord::A { domain , addr , ..} if domain == host => Some(*addr),
                _ => None,
            })
        })
        //Finally pick the first valid entry
        .next()
    }
    /// However, not all name servers are as that nice. In certain cases there won't
    /// be any A records in the additional section, and we'll have to perform *another*
    /// lookup in the midst. For this, we introduce a method for returning the host
    /// name of an appropriate name server.
    pub fn get_unresolved_ns<'a>(&'a self, qname: &'a str) -> Option<&'a str> {
        // Get an iterator over the nameservers in the authorities section
        self.get_ns(qname)
            .map(|(_, host)| host)
            // Finally, pick the first valid entry
            .next()
    }
}