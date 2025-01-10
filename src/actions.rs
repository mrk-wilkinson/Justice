use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestActionType {
    Keystrokes,
    ScreenCapture,
    FileUpload,
    ShellResponse,
    SystemInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseActionType {
    CallAction,
    ShellCommand,
    FileDownload,
    FileUpload,
    Wait,
}
