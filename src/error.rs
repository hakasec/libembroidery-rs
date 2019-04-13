extern crate shh;

use std::mem::drop;
use std::io::Read;

pub struct EmbLogErrorListener {
    stdout: shh::ShhStdout,
}

impl Drop for EmbLogErrorListener {
    fn drop(&mut self) {
        drop(&self.stdout);
    }
}

impl EmbLogErrorListener {
    pub fn new() -> EmbLogErrorListener {
        EmbLogErrorListener {
            stdout: shh::stdout().unwrap(),
        }
    }

    pub fn error(&mut self) -> Option<String> {
        let mut buffer = Vec::new();
        self.stdout.read_to_end(&mut buffer).unwrap();
        if buffer.len() < 5 {
            return None;
        }
        match String::from_utf8_lossy(&buffer[..5]).to_string().as_ref() {
            "ERROR" => Some(String::from_utf8_lossy(&buffer[7..]).to_string()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let mut e = EmbLogErrorListener::new();
        println!("ERROR: something is broken");
        match e.error() {
            Some(s) => assert_eq!(s, "something is broken"),
            None => panic!("There should've been an error"),
        };
    }

    #[test]
    fn test_shh() {
        let mut shh = shh::stdout().unwrap();
        let mut v = Vec::new();
        println!("hello world");
        shh.read_to_end(&mut v).unwrap();

        let s = String::from_utf8_lossy(&v).to_string();
        assert_eq!("hello world", s);
    }
}