use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod auth_api;
pub use self::auth_api::{ AuthApi, AuthApiClient };
mod deprecated_api;
pub use self::deprecated_api::{ DeprecatedApi, DeprecatedApiClient };
mod orders_api;
pub use self::orders_api::{ OrdersApi, OrdersApiClient };
mod other_api;
pub use self::other_api::{ OtherApi, OtherApiClient };
mod securities_api;
pub use self::securities_api::{ SecuritiesApi, SecuritiesApiClient };
mod users_api;
pub use self::users_api::{ UsersApi, UsersApiClient };
mod v2orders_api;
pub use self::v2orders_api::{ V2ordersApi, V2ordersApiClient };
mod ws_orders_api;
pub use self::ws_orders_api::{ WsOrdersApi, WsOrdersApiClient };

pub mod configuration;
pub mod client;
