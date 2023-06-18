#[macro_use]
extern crate serde_derive;

extern crate chrono;
pub extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate url;
extern crate rust_decimal;


use std::fmt;
use chrono::{NaiveDate, NaiveDateTime, DateTime, FixedOffset, Utc, SecondsFormat};
pub use rust_decimal::Decimal;

pub mod apis;
pub mod models;
pub mod date_serializer;
pub mod date_serializer_opt;
pub mod datetime_serializer;
pub mod serialize_quoted_numbers;
pub mod serialize_quoted_numbers_opt;

//mod tests;  //Put testing data and token to tests before uncomment

pub trait OutlinePrint<'a>: fmt::Display {
    fn outline_print(&'a self) -> String {
        format!("{}", self)
    }
}

impl<'a> OutlinePrint<'a> for &'a str {
    fn outline_print(&'a self) -> String {
        format!("{}", self)
    }
}

impl<'a> OutlinePrint<'a> for Decimal {
    fn outline_print(&'a self) -> String {
        format!("{}", self)
    }
}
impl<'a> OutlinePrint<'a> for i32 {
    fn outline_print(&'a self) -> String {
        format!("{:?}", self)
    }
}
impl<'a> OutlinePrint<'a> for i64 {
    fn outline_print(&'a self) -> String {
        format!("{:?}", self)
    }
}

impl<'a> OutlinePrint<'a> for u32 {
    fn outline_print(&'a self) -> String {
        format!("{:?}", self)
    }
}
impl<'a> OutlinePrint<'a> for u64 {
    fn outline_print(&'a self) -> String {
        format!("{:?}", self)
    }
}

impl<'a> OutlinePrint<'a> for f32 {
    fn outline_print(&'a self) -> String {
        format!("{:?}", self)
    }
}
impl<'a> OutlinePrint<'a> for f64 {
    fn outline_print(&'a self) -> String {
        format!("{:?}", self)
    }
}

impl<'a> OutlinePrint<'a> for bool {
    fn outline_print(&'a self) -> String {
        format!("{:?}", self)
    }
}
impl<'a> OutlinePrint<'a> for chrono::DateTime<chrono::Utc> {
    fn outline_print(&'a self) -> String {
        format!("{}", self.to_rfc3339_opts(SecondsFormat::Secs, true))
    }
}
impl<'a> OutlinePrint<'a> for chrono::NaiveDate {
    fn outline_print(&'a self) -> String {
        self.format("%Y-%m-%d").to_string()
    }
}