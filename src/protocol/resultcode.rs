#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ResultCode {
    NoError = 0,
    FormErr = 1,
    ServFail = 2,
    NXDomain = 3,
    NotImp = 4,
    Refused = 5,
}

impl ResultCode {
    pub fn from_num(num: u8) -> ResultCode {
        match num {
            1 => ResultCode::FormErr,
            2 => ResultCode::ServFail,
            3 => ResultCode::NXDomain,
            4 => ResultCode::NotImp,
            5 => ResultCode::Refused,
            0 => ResultCode::NoError,
            _ => ResultCode::NoError,
        }
    }
}