/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.BodyrequestOrdersActionsStopLimitTvWarp;
import io.swagger.client.model.BodyrequestOrdersActionsStopMarketTvWarp;
import io.swagger.client.model.inline_response_400;
import io.swagger.client.model.orders_actions_400_CommandAPI;
import io.swagger.client.model.orders_actions_LimitMarket_CommandAPI;
import io.swagger.client.model.stoporderWarp;
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
 * API tests for V2ordersApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> V2ordersApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    V2ordersApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * Создание стоп заявки
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn command_api_v2clientordersactionsstop_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsStopMarketTvWarp = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let response: OrdersActionsLimitMarketCommandApi = api_client.command_api_v2clientordersactionsstop(body, x_alor_reqid).await.unwrap();
}
/**
 * Создание стоп-лимитной заявки
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn command_api_v2clientordersactionsstop_limit_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsStopLimitTvWarp = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let response: OrdersActionsLimitMarketCommandApi = api_client.command_api_v2clientordersactionsstop_limit(body, x_alor_reqid).await.unwrap();
}
/**
 * Изменение стоп-лимитной заявки
 *
 * Изменение стоп-лимитной заявки
 *
 */
#[tokio::test(core_threads = 3)]
async fn command_api_v2clientordersactionsstop_limitstop_order_id_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsStopLimitTvWarp = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let stop_order_id: i64 = serde_json::from_value(value).unwrap();
    let response: OrdersActionsLimitMarketCommandApi = api_client.command_api_v2clientordersactionsstop_limitstop_order_id(body, x_alor_reqid, stop_order_id).await.unwrap();
}
/**
 * Снятие заявки
 *
 * Снятие заявки с указанным идентификатором
 *
 */
#[tokio::test(core_threads = 3)]
async fn command_api_warp_v2clientordersdelete_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let order_id: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let stop: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let json_response: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: Format = serde_json::from_value(value).unwrap();
    let response: String = api_client.command_api_warp_v2clientordersdelete(order_id, portfolio, exchange, stop, json_response, format).await.unwrap();
}
/**
 * Получение информации о стоп-заявках
 *
 * Запрос информации о всех стоп-заявках
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_get_all_stop_orders_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Vec<StoporderWarp> = api_client.dev_get_all_stop_orders(exchange, portfolio, format).await.unwrap();
}
/**
 * Получение информации о выбранной стоп-заявке
 *
 * Запрос информации о выбранной стоп-заявке
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_get_one_stop_order_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let order_id: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: Format = serde_json::from_value(value).unwrap();
    let response: StoporderWarp = api_client.dev_get_one_stop_order(exchange, portfolio, order_id, format).await.unwrap();
}
