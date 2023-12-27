/*
package io.swagger.client.api;

import .ApiException;
*/
use crate::models::*;
use crate::apis::*;
use crate::apis::configuration::Configuration;
use hyper::{
    client::connect::dns::GaiResolver,
    client::{Client, HttpConnector}
};
use std::sync::Arc;
use serde_json::json;

/**
 * API tests for OtherApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> OtherApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    OtherApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * Запрос текущего UTC времени в формате Unix
 *
 * **Запрос может быть выполнен без авторизации**. При отправке анонимного запроса вернутся данные, бывшие актуальными 15 минут назад.  Запрос текущего UTC времени в формате Unix Time Seconds. Если этот запрос выполнен без авторизации, то будет возвращено время, которое было 15 минут назад. 
 *
 */
#[tokio::test(core_threads = 3)]
async fn local_time_test() {
    let api_client = get_client();
    let response: i64 = api_client.local_time().await.unwrap();
}
