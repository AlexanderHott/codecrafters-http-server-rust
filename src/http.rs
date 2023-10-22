#![allow(dead_code, unused_variables)]
use anyhow::anyhow;
use std::{collections::HashMap, str::FromStr};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Method {
    Get,
}

impl FromStr for Method {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "GET" => Ok(Method::Get),
            m => Err(anyhow!("Not implemented for {m}")),
        };
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum HttpVersion {
    Http11,
}

impl FromStr for HttpVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "HTTP/1.1" => Ok(HttpVersion::Http11),
            v => Err(anyhow!("Not implemented for {}", v)),
        };
    }
}

#[derive(Debug)]
pub struct StatusLine {
    pub method: Method,
    pub path: String,
    pub version: HttpVersion,
}

impl FromStr for StatusLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_ascii_whitespace();
        let method = Method::from_str(words.next().unwrap())?;
        let path = words.next().unwrap();
        let version = HttpVersion::from_str(words.next().unwrap())?;
        return Ok(Self {
            method,
            path: path.to_string(),
            version,
        });
    }
}

#[derive(Debug)]
pub struct Headers(pub HashMap<String, String>);

impl FromStr for Headers {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hm = HashMap::new();
        eprintln!("Parsing headers {s}");

        for line in s.split("\r\n").collect::<Vec<_>>() {
            if let Some((key, val)) = line.split_once(": ") {
                hm.entry(key.to_string())
                    .and_modify(|s| *s = val.to_string())
                    .or_insert(val.to_string());
            }
        }

        Ok(Self(hm))
    }
}
