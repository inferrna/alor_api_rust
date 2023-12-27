/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.BodyrequestOrdersActionsLimitTv;
import io.swagger.client.model.BodyrequestOrdersActionsMarketTVput;
import io.swagger.client.model.BodyrequestOrdersActionsMarketTv;
import io.swagger.client.model.EstimateOrderViewModel;
import io.swagger.client.model.estimateOrderModel;
import io.swagger.client.model.estimateOrderViewModel;
import io.swagger.client.model.inline_response_400;
import io.swagger.client.model.order;
import io.swagger.client.model.orders_actions_400_CommandAPI;
import io.swagger.client.model.orders_actions_LimitMarket;
import io.swagger.client.model.orders_actions_LimitMarket_CommandAPI;
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
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос создаёт на бирже новую заявку на покупку или продажу торгового инструмента по указанной в теле запроса цене. 
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
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос изменяет характеристики ранее поданной лимитной заявки с указанным номером 
 *
 */
#[tokio::test(core_threads = 3)]
async fn command_api_v2clientordersactionslimitput_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BodyrequestOrdersActionsLimitTv = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let x_alor_reqid: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let order_id: i64 = serde_json::from_value(value).unwrap();
    let response: OrdersActionsLimitMarket = api_client.command_api_v2clientordersactionslimitput(body, x_alor_reqid, order_id).await.unwrap();
}
/**
 * Создание рыночной заявки
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос создаёт на бирже новую заявку на покупку или продажу торгового инструмента по рыночной цене. 
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
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Запрос изменяет характеристики ранее поданной рыночной заявки с указанным номером 
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
    let order_id: i64 = serde_json::from_value(value).unwrap();
    let response: OrdersActionsLimitMarket = api_client.command_api_v2clientordersactionsmarketput(body, x_alor_reqid, order_id).await.unwrap();
}
/**
 * Снятие заявки
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;.  Снятие заявки с указанным идентификатором 
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
 * Провести оценку одной заявки
 *
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;. 
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
 * **Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок &#x60;Authorization&#x60; со значением &#x60;Bearer &lt;ваш JWT&gt;&#x60;. 
 *
 */
#[tokio::test(core_threads = 3)]
async fn v2clientordersactionsestimateall_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: Vec<EstimateOrderViewModel> = serde_json::from_value(value).unwrap();
    let response: Vec<EstimateOrderModel> = api_client.v2clientordersactionsestimateall(body).await.unwrap();
}
