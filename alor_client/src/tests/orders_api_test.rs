/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.BodyrequestOrdersActionsLimitTVput;
import io.swagger.client.model.BodyrequestOrdersActionsLimitTv;
import io.swagger.client.model.BodyrequestOrdersActionsMarketTVput;
import io.swagger.client.model.BodyrequestOrdersActionsMarketTv;
import io.swagger.client.model.BodyrequestOrdersActionsStop;
import io.swagger.client.model.BodyrequestOrdersActionsStoplimit;
import io.swagger.client.model.EstimateOrderViewModel;
import io.swagger.client.model.estimateOrderModel;
import io.swagger.client.model.estimateOrderViewModel;
import io.swagger.client.model.inline_response_400;
import io.swagger.client.model.orders_actions_400;
import io.swagger.client.model.orders_actions_400_CommandAPI;
import io.swagger.client.model.orders_actions_LimitMarket;
import io.swagger.client.model.orders_actions_LimitMarket_CommandAPI;
import io.swagger.client.model.orders_actions_StopProfitLoss;
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
 * API tests for OrdersApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> OrdersApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    OrdersApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * Создание лимитной заявки
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn command_api_v2clientordersactionslimit_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsLimitTv = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let response: OrdersActionsLimitMarketCommandApi = api_client.command_api_v2clientordersactionslimit(body, x_alor_reqid).await.unwrap();
}
/**
 * Изменение лимитной заявки
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn command_api_v2clientordersactionslimitput_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsLimitTVput = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let order_id: String = serde_json::from_value(value).unwrap();
    let response: OrdersActionsLimitMarket = api_client.command_api_v2clientordersactionslimitput(body, x_alor_reqid, order_id).await.unwrap();
}
/**
 * Создание рыночной заявки
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn command_api_v2clientordersactionsmarket_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsMarketTv = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let response: OrdersActionsLimitMarketCommandApi = api_client.command_api_v2clientordersactionsmarket(body, x_alor_reqid).await.unwrap();
}
/**
 * Изменение рыночной заявки
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn command_api_v2clientordersactionsmarketput_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsMarketTVput = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let order_id: String = serde_json::from_value(value).unwrap();
    let response: OrdersActionsLimitMarket = api_client.command_api_v2clientordersactionsmarketput(body, x_alor_reqid, order_id).await.unwrap();
}
/**
 * Снятие заявки
 *
 * Снятие заявки с указанным идентификатором
 *
 */
#[tokio::test(core_threads = 3)]
async fn command_api_v2clientordersdelete_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let order_id: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let stop: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let json_response: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: JsonFormat = serde_json::from_value(value).unwrap();
    let response: OrdersActionsDeleteOrderIdCommandApi = api_client.command_api_v2clientordersdelete(order_id, portfolio, exchange, stop, json_response, format).await.unwrap();
}
/**
 * Провести оценку одной заявки
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn v2clientordersactionsestimate_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: EstimateOrderViewModel = serde_json::from_value(value).unwrap();
    let response: EstimateOrderModel = api_client.v2clientordersactionsestimate(body).await.unwrap();
}
/**
 * Провести оценку нескольких заявок
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn v2clientordersactionsestimateall_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: Vec<EstimateOrderViewModel> = serde_json::from_value(value).unwrap();
    let response: Vec<EstimateOrderModel> = api_client.v2clientordersactionsestimateall(body).await.unwrap();
}
/**
 * Снятие стоп-заявки
 *
 * Снятие стоп-заявки с указанным идентификатором
 *
 */
#[tokio::test(core_threads = 3)]
async fn v2clientordersactionsorder_id_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let trade_server_code: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let order_id: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let stop: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let response: OrdersActionsDeleteOrderId = api_client.v2clientordersactionsorder_id(trade_server_code, order_id, portfolio, stop, x_alor_reqid).await.unwrap();
}
/**
 * Создание стоп-лосс заявки
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn v2clientordersactionsstop_loss_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsStop = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let trade_server_code: String = serde_json::from_value(value).unwrap();
    let response: OrdersActionsStopProfitLoss = api_client.v2clientordersactionsstop_loss(body, x_alor_reqid, trade_server_code).await.unwrap();
}
/**
 * Создание стоп-лосс лимит заявки
 *
 * Создание стоп-лосс лимит заявки
 *
 */
#[tokio::test(core_threads = 3)]
async fn v2clientordersactionsstop_loss_limit_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsStoplimit = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let trade_server_code: String = serde_json::from_value(value).unwrap();
    let response: OrdersActionsStopProfitLoss = api_client.v2clientordersactionsstop_loss_limit(body, x_alor_reqid, trade_server_code).await.unwrap();
}
/**
 * Изменение стоп-лосс лимит заявки
 *
 * Изменение стоп-лосс лимит заявки с указанным номером
 *
 */
#[tokio::test(core_threads = 3)]
async fn v2clientordersactionsstop_loss_limitorder_id_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsStoplimit = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let trade_server_code: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let order_id: i32 = serde_json::from_value(value).unwrap();
    let response: OrdersActionsStopProfitLoss = api_client.v2clientordersactionsstop_loss_limitorder_id(body, x_alor_reqid, trade_server_code, order_id).await.unwrap();
}
/**
 * Изменение стоп-лосс заявки
 *
 * Изменение стоп-лосс заявки с указанным номером
 *
 */
#[tokio::test(core_threads = 3)]
async fn v2clientordersactionsstop_lossorder_id_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsStop = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let trade_server_code: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let order_id: i32 = serde_json::from_value(value).unwrap();
    let response: OrdersActionsStopProfitLoss = api_client.v2clientordersactionsstop_lossorder_id(body, x_alor_reqid, trade_server_code, order_id).await.unwrap();
}
/**
 * Создание стоп-заявки
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn v2clientordersactionstake_profit_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsStop = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let trade_server_code: String = serde_json::from_value(value).unwrap();
    let response: OrdersActionsStopProfitLoss = api_client.v2clientordersactionstake_profit(body, x_alor_reqid, trade_server_code).await.unwrap();
}
/**
 * Создание стоп-лимит заявки
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn v2clientordersactionstake_profit_limit_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsStoplimit = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let trade_server_code: String = serde_json::from_value(value).unwrap();
    let response: OrdersActionsStopProfitLoss = api_client.v2clientordersactionstake_profit_limit(body, x_alor_reqid, trade_server_code).await.unwrap();
}
/**
 * Изменение стоп-лимит заявки
 *
 * Изменение стоп-лимит заявки с указанным номером
 *
 */
#[tokio::test(core_threads = 3)]
async fn v2clientordersactionstake_profit_limitorder_id_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsStoplimit = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let trade_server_code: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let order_id: i32 = serde_json::from_value(value).unwrap();
    let response: OrdersActionsStopProfitLoss = api_client.v2clientordersactionstake_profit_limitorder_id(body, x_alor_reqid, trade_server_code, order_id).await.unwrap();
}
/**
 * Изменение стоп-заявки
 *
 * Изменение стоп-заявки с указанным номером
 *
 */
#[tokio::test(core_threads = 3)]
async fn v2clientordersactionstake_profitorder_id_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsStop = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let trade_server_code: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let order_id: i32 = serde_json::from_value(value).unwrap();
    let response: OrdersActionsStopProfitLoss = api_client.v2clientordersactionstake_profitorder_id(body, x_alor_reqid, trade_server_code, order_id).await.unwrap();
}
