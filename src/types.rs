use anyhow::{bail, Error as AnyError};
use std::str::FromStr;

pub enum State {
    NotAuthenticated,
    Authenticated,
    Selected,
}

#[derive(Debug)]
pub enum StatusResponse {
    Ok,
    No,
    Bad,
    Preauth,
    Bye,
}

impl FromStr for StatusResponse {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut args = s.split_whitespace().collect::<Vec<&str>>();
        if args.len() < 2 {
            bail!("Invalid response")
        } else {
            match args.remove(1) {
                "OK" => Ok(Self::Ok),
                "BAD" => Ok(Self::Bad),
                "NO" => Ok(Self::No),
                _ => bail!("Invalid response"),
            }
        }
    }
}

pub struct Commands;

impl Commands {
    pub const CAPABILITY: &'static [u8] = "CAPABILITY".as_bytes();
    pub const LOGOUT: &'static [u8] = "LOGOUT".as_bytes();
    pub const NOOP: &'static [u8] = "NOOP".as_bytes();
    pub const STARTTLS: &'static [u8] = "STARTTLS".as_bytes();
    // TODO: AUTHENTICATE
}
