extern crate reqwest;

pub mod metadata;

type Result<T> = std::result::Result<T, reqwest::Error>;