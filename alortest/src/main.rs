use std::sync::Arc;
use alor_api::apis::configuration::Configuration;
use alor_api::apis::UsersApiClient;
use alor_api::apis::UsersApi;
use alor_api::hyper;
use alor_api::hyper::{Body, Client};
use alor_api::hyper::body::HttpBody;
use alor_api::hyper::client::connect::dns::GaiResolver;
use alor_api::hyper::client::HttpConnector;
use alor_api::models::{Exchange, JsonFormat};
use hyper_tls::HttpsConnector;
use std::borrow::Borrow;
use std::env;
use serde_derive::{Serialize, Deserialize};


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

async fn init_meta_client(refresh_token: &str) -> Arc<Configuration<HttpsConnector<HttpConnector<GaiResolver>>>> {
    //Init market https client
    let token = get_token(refresh_token).await;

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
    let meta_client = init_meta_client(refresh_token.as_str()).await;
    let users_client = UsersApiClient::new(meta_client);
    //let pf = users_client.dev_user_portfolio("D83884").await.unwrap();
    let pf_summary = users_client.exchange_portfolio_summary(Exchange::SPBX, "D83884", Some(JsonFormat::SIMPLE)).await.unwrap();
    dbg!(pf_summary);
}
