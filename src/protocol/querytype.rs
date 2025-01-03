#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum QueryType {
    Unknown(u16),
    A, // 1
    NS, // 2
    Cname, // 5
    Soa, // 6
    MX , // 15
    Txt, // 16
    Aaaa, // 28
}

impl QueryType {
    pub fn to_num(self) -> u16 {
        match self {
            QueryType::Unknown(x) => x,
            QueryType::A => 1,
            QueryType::NS => 2,
            QueryType::Cname => 5,
            QueryType::Soa => 6,
            QueryType::MX => 15,
            QueryType::Txt => 16,
            QueryType::Aaaa => 28,
        }
    }

    pub fn from_num(num: u16) -> QueryType {
        match num {
            1 => QueryType::A,
            2 => QueryType::NS,
            5 => QueryType::Cname,
            6 => QueryType::Soa,
            15 => QueryType::MX,
            16 => QueryType::Txt,
            28 => QueryType::Aaaa,
            _ => QueryType::Unknown(num),
        }
    }
}