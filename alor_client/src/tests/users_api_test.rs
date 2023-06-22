/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.fortsrisk;
import io.swagger.client.model.money;
import io.swagger.client.model.order;
import io.swagger.client.model.position;
import io.swagger.client.model.risk;
import io.swagger.client.model.servers_info;
import io.swagger.client.model.stoporderWarp;
import io.swagger.client.model.summary;
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
 * API tests for UsersApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> UsersApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    UsersApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * Получение информации о всех заявках
 *
 * Запрос информации о всех заявках
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_get_all_orders_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: JsonFormat = serde_json::from_value(value).unwrap();
    let response: Vec<Order> = api_client.dev_get_all_orders(exchange, portfolio, format).await.unwrap();
}
/**
 * Получение информации о позициях
 *
 * Запрос информации о позициях
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_get_all_positions_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: JsonFormat = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let without_currency: bool = serde_json::from_value(value).unwrap();
    let response: Vec<Position> = api_client.dev_get_all_positions(exchange, portfolio, format, without_currency).await.unwrap();
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
    let format: JsonFormat = serde_json::from_value(value).unwrap();
    let response: Vec<StoporderWarp> = api_client.dev_get_all_stop_orders(exchange, portfolio, format).await.unwrap();
}
/**
 * Получение информации о сделках
 *
 * Запрос информации о сделках (только за текущую торговую сессию)
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_get_all_trades_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: JsonFormat = serde_json::from_value(value).unwrap();
    let response: Vec<Trade> = api_client.dev_get_all_trades(exchange, portfolio, format).await.unwrap();
}
/**
 * Получение информации о выбранной заявке
 *
 * Запрос информации о выбранной заявке
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_get_one_order_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let order_id: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: JsonFormat = serde_json::from_value(value).unwrap();
    let response: Order = api_client.dev_get_one_order(exchange, portfolio, order_id, format).await.unwrap();
}
/**
 * Получение информации о позициях выбранного инструмента
 *
 * Запрос информации о позициях выбранного инструмента
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_get_one_position_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let symbol: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: JsonFormat = serde_json::from_value(value).unwrap();
    let response: Position = api_client.dev_get_one_position(exchange, portfolio, symbol, format).await.unwrap();
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
    let order_id: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: JsonFormat = serde_json::from_value(value).unwrap();
    let response: StoporderWarp = api_client.dev_get_one_stop_order(exchange, portfolio, order_id, format).await.unwrap();
}
/**
 * Получение информации о сделках по выбранному инструменту
 *
 * Запрос информации о сделках по выбранному инструменту
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_get_ticker_trades_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let ticker: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: JsonFormat = serde_json::from_value(value).unwrap();
    let response: Vec<Trade> = api_client.dev_get_ticker_trades(exchange, portfolio, ticker, format).await.unwrap();
}
/**
 * Получение списка серверов портфелей
 *
 * Получение списка серверов. В ответе в поле tradeServerCode содержится значение которое надо использовать. Не являются частью API торговой системы.
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_user_portfolio_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let username: String = serde_json::from_value(value).unwrap();
    let response: ServersInfo = api_client.dev_user_portfolio(username).await.unwrap();
}
/**
 * Получение информации по деньгам для выбранного портфеля
 *
 * Запрос информации о позиции по деньгам. Вызов существует для обратной совместимости с API v1, предпочтительно использовать другие вызовы (/summary, /risk, /positions)
 *
 */
#[tokio::test(core_threads = 3)]
async fn exchange_portfolio_money_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: JsonFormat = serde_json::from_value(value).unwrap();
    let response: Money = api_client.exchange_portfolio_money(exchange, portfolio, format).await.unwrap();
}
/**
 * Получение информации о портфеле
 *
 * Запрос сводной информации о портфеле
 *
 */
#[tokio::test(core_threads = 3)]
async fn exchange_portfolio_summary_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: JsonFormat = serde_json::from_value(value).unwrap();
    let response: Summary = api_client.exchange_portfolio_summary(exchange, portfolio, format).await.unwrap();
}
/**
 * Получение информации о рисках на срочном рынке
 *
 * Запрос информации о рисках на срочном рынке для выбранного портфеля
 *
 */
#[tokio::test(core_threads = 3)]
async fn fortsrisk_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: JsonFormat = serde_json::from_value(value).unwrap();
    let response: Fortsrisk = api_client.fortsrisk(exchange, portfolio, format).await.unwrap();
}
/**
 * Получение информации о рисках
 *
 * Запрос информации о рисках
 *
 */
#[tokio::test(core_threads = 3)]
async fn risk_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: JsonFormat = serde_json::from_value(value).unwrap();
    let response: Risk = api_client.risk(exchange, portfolio, format).await.unwrap();
}
/**
 * Получение истории сделок
 *
 * Запрос списка сделок за предыдущие дни (не более 1000 сделок за один запрос)
 *
 */
#[tokio::test(core_threads = 3)]
async fn trade_stats_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let date_from: NaiveDate = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let from: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let limit: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let descending: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: JsonFormat = serde_json::from_value(value).unwrap();
    let response: Vec<Trade> = api_client.trade_stats(exchange, portfolio, date_from, from, limit, descending, format).await.unwrap();
}
/**
 * Получение истории сделок (один тикер)
 *
 * Запрос списка сделок за предыдущие дни (не более 1000 сделок за один запрос) по одному инструменту. 
 *
 */
#[tokio::test(core_threads = 3)]
async fn trade_stats_by_symbol_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let portfolio: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let symbol: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let date_from: NaiveDate = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let from: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let limit: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let descending: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: JsonFormat = serde_json::from_value(value).unwrap();
    let response: Vec<Trade> = api_client.trade_stats_by_symbol(exchange, portfolio, symbol, date_from, from, limit, descending, format).await.unwrap();
}
