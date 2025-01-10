use serde::{Serialize, Deserialize};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub implantid: String,
    pub messageheaders: Vec<String>,
    pub messagecontent: String,
    pub timestamp: u64,
}

impl Request {
    pub fn new(implantid: String, messageheaders: Vec<String>, messagecontent: String, timestamp: u64) -> Self {
        Request {
            implantid,
            messageheaders,
            messagecontent,
            timestamp,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Keystroke {
    pub key: String,
    pub timestamp: u64,
}

impl Keystroke {
    pub fn new(key: String, timestamp: u64) -> Self {
        Keystroke { key, timestamp }
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
