use std::str::FromStr;

pub enum Method {
    GET,
    PUT,
    POST,
    DELETE,
    HEAD,
    PATCH,
    OPTIONS,
    CONNECT,
    TRACE
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET)
        }
    }
}

pub struct MethodError;