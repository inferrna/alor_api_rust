/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.alltradeshistory;
import io.swagger.client.model.history;
import io.swagger.client.model.orderbook;
import io.swagger.client.model.riskRates;
import io.swagger.client.model.security;
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
 * API tests for SecuritiesApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> SecuritiesApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    SecuritiesApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * Запрос истории для выбранных биржи и инструмента
 *
 * **Запрос может быть выполнен без авторизации**. При отправке анонимного запроса вернутся данные, бывшие актуальными 15 минут назад.  Запрос истории рынка для выбранных биржи и финансового инструмента. Данные имеют задержку в 15 минут, если запрос не авторизован. Для авторизованных клиентов задержка не применяется. 
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_history_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let symbol: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let tf: Duration = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let from: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let to: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let untraded: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: Format = serde_json::from_value(value).unwrap();
    let response: History = api_client.dev_history(symbol, exchange, tf, from, to, untraded, format).await.unwrap();
}
/**
 * Получение информации о биржевом стакане
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос биржевого стакана 
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_orderbook_exchang_seccode_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let seccode: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let depth: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Orderbook = api_client.dev_orderbook_exchang_seccode(exchange, seccode, depth, format).await.unwrap();
}
/**
 * Получение информации о котировках для выбранных инструментов
 *
 * **Запрос может быть выполнен без авторизации**. При отправке анонимного запроса вернутся данные, бывшие актуальными 15 минут назад.  Запрос информации о котировках для выбранных инструментов и бирж 
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_quotes_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let symbols: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Vec<Symbol> = api_client.dev_quotes(symbols, format).await.unwrap();
}
/**
 * Получение котировки по ближайшему фьючерсу (код)
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос котировки по ближайшему фьючерсу (только по коду, без даты) 
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_securities_futures_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let symbol: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Vec<Symbol> = api_client.dev_securities_futures(exchange, symbol, format).await.unwrap();
}
/**
 * Получение информации о торговых инструментах
 *
 * **Запрос может быть выполнен без авторизации**. При отправке анонимного запроса вернутся данные, бывшие актуальными 15 минут назад.  Запрос информации о торговых инструментах 
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_securities_search_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let query: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let limit: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let offset: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let sector: Market = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let cficode: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Vec<Security> = api_client.dev_securities_search(query, limit, offset, sector, cficode, exchange, format).await.unwrap();
}
/**
 * Получение информации о всех сделках по ценным бумагам за сегодня
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запросить данные о всех сделках (лента) по ценным бумагам за сегодняшний день 
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_securities_search_all_trades_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let symbol: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: Format = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let from: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let to: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let from_id: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let to_id: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let take: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let descending: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let include_virtual_trades: bool = serde_json::from_value(value).unwrap();
    let response: Vec<Alltrade> = api_client.dev_securities_search_all_trades(exchange, symbol, format, from, to, from_id, to_id, take, descending, include_virtual_trades).await.unwrap();
}
/**
 * Получение исторической информации о всех сделках по ценным бумагам
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запросить данные о сделках (лента) по ценным бумагам за исторический период (за текущий день сделки не отдаются) 
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_securities_search_all_trades_history_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let symbol: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let limit: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let from: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let to: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let offset: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Alltradeshistory = api_client.dev_securities_search_all_trades_history(exchange, symbol, limit, from, to, offset, format).await.unwrap();
}
/**
 * Получение информации о торговых инструментах на выбранной бирже
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос информации об инструментах на выбранной бирже 
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_securities_search_exchange_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: Format = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let market: Market = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let include_old: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let limit: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let offset: i32 = serde_json::from_value(value).unwrap();
    let response: Vec<Security> = api_client.dev_securities_search_exchange(exchange, format, market, include_old, limit, offset).await.unwrap();
}
/**
 * Получение информации о выбранном финансовом инструменте
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос информации о выбранном финансовом инструменте на бирже 
 *
 */
#[tokio::test(core_threads = 3)]
async fn dev_securities_search_exchange_code_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let symbol: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let format: Format = serde_json::from_value(value).unwrap();
    let response: Security = api_client.dev_securities_search_exchange_code(exchange, symbol, format).await.unwrap();
}
/**
 * Запрос ставок риска
 *
 * **Запрос может быть выполнен без авторизации**. При отправке анонимного запроса вернутся данные, бывшие актуальными 15 минут назад.  Получение ставок риска для маржинальной торговли. 
 *
 */
#[tokio::test(core_threads = 3)]
async fn risk_rates_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let exchange: Exchange = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let ticker: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let risk_category_id: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let search: String = serde_json::from_value(value).unwrap();
    let response: RiskRates = api_client.risk_rates(exchange, ticker, risk_category_id, search).await.unwrap();
}
