/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.fortsrisk;
import io.swagger.client.model.order;
import io.swagger.client.model.position;
import io.swagger.client.model.risk;
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
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Возвращает информацию о всех заявках для указанного &#x60;portfolio&#x60;, созданных на заданной в параметре &#x60;exchange&#x60; бирже. 
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
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Vec<Order> = api_client.dev_get_all_orders(exchange, portfolio, format).await.unwrap();
}
/**
 * Получение информации о позициях
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Возвращает информацию обо всех позициях указанного &#x60;portfolio&#x60;. 
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
    let format: Format = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let without_currency: bool = serde_json::from_value(value).unwrap();
    let response: Vec<Position> = api_client.dev_get_all_positions(exchange, portfolio, format, without_currency).await.unwrap();
}
/**
 * Получение информации о стоп-заявках
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос информации о всех стоп-заявках 
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
 * Получение информации о сделках
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос информации о сделках (только за текущую торговую сессию) 
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
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Vec<Trade> = api_client.dev_get_all_trades(exchange, portfolio, format).await.unwrap();
}
/**
 * Получение информации о выбранной заявке
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Возвращает информацию о выбранной в параметре &#x60;orderId&#x60; заявке. 
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
    let order_id: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Order = api_client.dev_get_one_order(exchange, portfolio, order_id, format).await.unwrap();
}
/**
 * Получение информации о позициях выбранного инструмента
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос информации о позициях выбранного инструмента 
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
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Position = api_client.dev_get_one_position(exchange, portfolio, symbol, format).await.unwrap();
}
/**
 * Получение информации о выбранной стоп-заявке
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос информации о выбранной стоп-заявке 
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
/**
 * Получение информации о сделках по выбранному инструменту
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос информации о сделках по выбранному инструменту 
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
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Vec<Trade> = api_client.dev_get_ticker_trades(exchange, portfolio, ticker, format).await.unwrap();
}
/**
 * Получение информации о портфеле
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос сводной информации о портфеле 
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
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Summary = api_client.exchange_portfolio_summary(exchange, portfolio, format).await.unwrap();
}
/**
 * Получение информации о рисках на срочном рынке
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос информации о рисках на срочном рынке для выбранного портфеля 
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
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Fortsrisk = api_client.fortsrisk(exchange, portfolio, format).await.unwrap();
}
/**
 * Получение информации о рисках
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос информации о рисках 
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
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Risk = api_client.risk(exchange, portfolio, format).await.unwrap();
}
/**
 * Получение истории сделок
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос списка сделок за предыдущие дни (не более 1000 сделок за один запрос) 
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
    let from: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let limit: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let descending: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Vec<Trade> = api_client.trade_stats(exchange, portfolio, date_from, from, limit, descending, format).await.unwrap();
}
/**
 * Получение истории сделок (один тикер)
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос списка сделок за предыдущие дни (не более 1000 сделок за один запрос) по одному инструменту.  
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
    let from: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let limit: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let descending: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Vec<Trade> = api_client.trade_stats_by_symbol(exchange, portfolio, symbol, date_from, from, limit, descending, format).await.unwrap();
}
