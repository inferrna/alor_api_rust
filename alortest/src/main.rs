use std::sync::Arc;
use alor_api::apis::configuration::Configuration;
use alor_api::apis::UsersApiClient;
use alor_api::apis::UsersApi;
use alor_api::hyper;
use alor_api::hyper::{Body, Client};
use alor_api::hyper::body::HttpBody;
use alor_api::hyper::client::connect::dns::GaiResolver;
use alor_api::hyper::client::HttpConnector;
use alor_api::models::{Exchange, Format};
use hyper_tls::HttpsConnector;
use std::borrow::Borrow;
use std::env;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use serde_derive::{Serialize, Deserialize};
use serde::de;
use base64::{Engine as _, alphabet, engine::{self, general_purpose}, decoded_len_estimate};
use base64::engine::{DecodePaddingMode, GeneralPurposeConfig};
use regex::Regex;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct TokenKeeper {
    AccessToken: String
}

async fn get_token(refresh_token: &str) -> String {
    let mut req =
        hyper::Request::builder()
            .method("POST")
            .uri(format!("https://oauth.alor.ru/refresh?token={refresh_token}"));

    let headers = req.headers_mut().unwrap();

    headers.insert(hyper::header::CONTENT_LENGTH, "0".parse().unwrap());

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, alor_api::hyper::Body>(https);
    let mut conf = Configuration::new(client);

    let somebody = Body::empty();

    let req = req.body(somebody).unwrap();

    let mut res = conf
        .client
        .request(req)
        .await
        .unwrap();

    let mut res_body: Vec<u8> = vec![];

    while let Some(chunk) = res.body_mut().data().await {
        let mut chunk_vec = chunk.unwrap().to_vec();
        res_body.append(chunk_vec.as_mut());
    }

    let tk: TokenKeeper = serde_json::  from_slice(res_body.borrow()).unwrap();
    tk.AccessToken
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct JwtDecodedInfo {
    sub: String,
    #[serde(deserialize_with = "deserialize_stupid_spaced_array")]
    portfolios: Vec<String>,
    exp: i64,
    iat: i64
}

impl JwtDecodedInfo {
    fn get_matched_pf(&self, start_sym: char ) -> Option<&String> {
        self.portfolios.iter().find(|s|s.starts_with(start_sym))
    }
}

fn deserialize_stupid_spaced_array<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: de::Deserializer<'de>,
{
    // define a visitor that deserializes
    // `ActualData` encoded as json within a string
    struct JsonStringVisitor;

    impl<'de> de::Visitor<'de> for JsonStringVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("A string with spaces")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
        {
            Ok(v.split(' ').map(|s|s.to_string()).collect())
        }
    }

    // use our visitor to deserialize an `ActualValue`
    deserializer.deserialize_any(JsonStringVisitor)
}

fn get_portfolios(_jwt_token: &str) -> JwtDecodedInfo {
    let tokens_parts: Vec<&str> = _jwt_token.split('.').collect();
    let decoder = engine::GeneralPurpose::new(
        &alphabet::STANDARD,
        GeneralPurposeConfig::new()
            .with_decode_allow_trailing_bits(true)
            .with_decode_padding_mode(DecodePaddingMode::Indifferent));

    let info_token_encoded = tokens_parts[1];
    let info_token_bytes = decoder.decode(info_token_encoded).unwrap();
    let info_token_str = String::from_utf8(info_token_bytes).unwrap();

    serde_json::from_str(&info_token_str).unwrap()
}

async fn init_meta_client(token: String) -> Arc<Configuration<HttpsConnector<HttpConnector<GaiResolver>>>> {
    //Init market https client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, alor_api::hyper::Body>(https);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(token); //Must be provided
    conf.base_path = "https://api.alor.ru".to_string(); //Must be provided
    let conf = Arc::new(conf);
    conf
}

#[tokio::main(flavor="current_thread")]
async fn main() {
    let refresh_token = env::args().skip(1).last().expect("Предоставьте refresh_token как параметр запуска");
    let jwt_token = get_token(&refresh_token).await;
    let jwt_info = get_portfolios(&jwt_token);
    dbg!(&jwt_info);
    let meta_client = init_meta_client(jwt_token).await;
    let users_client = UsersApiClient::new(meta_client);

    if let Some(pf_id) = jwt_info.get_matched_pf('D') {
        let pf_summary = users_client.exchange_portfolio_summary(Exchange::SPBX, pf_id, Some(Format::SIMPLE)).await.unwrap();
        dbg!(pf_summary);
        let pf_summary = users_client.dev_get_all_positions(Exchange::SPBX, pf_id, None, None).await.unwrap();
        dbg!(pf_summary);
    }
}
