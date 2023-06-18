/* 
 * Alor OpenAPI V2
 *
 * API для работы с торговой системой АЛОР Брокер. Предоставляет интерфейсы для выставления заявок и получения биржевой информации.  Данные для неавторизованных запросов предоставляются с задержкой от 15 минут, для авторизованных - без задержек.   Публичная биржевая информация может быть получена через HTTP(S) API, а также доступна через однократно установленное WebSocket соединение. <br> **Внимание!** WebSocket соединения могут и будут разрываться *(например, если клиент не успевает обрабатывать сообщения и на стороне API в буфере накопится более 5000 событий)*. <br> Во внешнем ПО необходимо предусмотреть механизмы переподключения и переподписки (при необходимости)! <br> <br>  В OpenAPI V2 доступны \"Московская Биржа\" (MOEX) и \"Биржа СПБ\" (SPBX).   <h4> Доступные типы данных </h4>  * Все сделки  * Все заявки  * Информация по инструментам  * Котировки  * Биржевые стаканы  * Исторические данные  * Позиции  * Информация о клиенте  <h4>Поддерживаемые виды заявок</h4>  * рыночные  * лимитные  * стоп-лосс  * тейк-профит  * стоп-лосс лимит  * тейк-профит лимит  <h4>    Ограничения по частоте запросов     </h4> <p>На текущий момент ограничений по количеству запросов в минуту нет, однако есть ограничение на общее количество подписок (сотни тысяч). При достижении лимита подписок клиент будет заблокирован и в течение нескольких минут не сможет создавать новые подписки. Уже существующие подписки продолжат работать. <br/>  Сервер может обрабатывать \"тяжелые\" запросы (пример - история за все время) и запросы без авторизации с меньшим приоритетом.<br/> <br/></p>   <h2> Авторизация </h2>  <h4>OAuth</h4>  <b>Внимание!</b>   JWT и refresh token — равносильны логину и паролю. Их нужно скрывать от публичного доступа.  <h4>Для разработчиков сторонних приложений, в которых торговлю будут вести их пользователи.</h4>  Мы предоставляем сервис для авторизации сторониих приложений по стандарту OAuth 2.0. С примером приложения, использующего OAuth сервис для авторизации пользователей можно ознакомиться в разделе  <a href=\"/examples\">Примеры</a>.  Список разрешений (scopes), которые могут быть выданы приложению: <table>   <tr>     <td><b>OrdersRead</b></td>     <td>Чтение выставленных заявок</td>   </tr>   <tr>     <td><b>OrdersCreate</b></td>     <td>Выставление заявок</td>   </tr>   <tr>     <td><b>Trades</b></td>     <td>Чтение совершенных сделок</td>   </tr>   <tr>     <td><b>Personal</b></td>     <td>Персональная информация: ФИО, почта и т.п.</td>   </tr>   <tr>     <td><b>Stats</b></td>     <td>Статистика: прибыль, средние цены и т.п.</td>   </tr> </table>  <h4>Для ведения операций от своего имени</h4>  Выписать себе <b>refresh token</b> для ведения операций от своего имени [можно здесь](https://alor.dev/open-api-tokens).  <h2>Краткое описание работы с авторизацией</h2>  Чтобы выполнить авторизованный запрос, добавьте в запрос заголовок с именем \"Authorization\" и значением, состоящим из префикса `\"Bearer \"` и валидного JWT токена.  Срок жизни JWT короткий: это сделано для безопасности.  Для большинства вариантов использования API мы рекоммендуем использовать механизм  <b>refresh token</b> .  Механизм  <b>refresh token</b>  позволяет получать JWT с новым сроком жизни. Для этого отправьте POST запрос на адрес `https://oauthdev.alor.ru/refresh?token={refreshToken}` *(тестовый контур)* или `https://oauth.alor.ru/refresh?token={refreshToken}` *(боевой контур)*. Если у  <b>refresh token</b>  не истек срок жизни и не он не был отозван, то в теле ответа в поле AccessToken вернётся свежий JWT токен.   Срок жизни  <b>refresh token</b>, получаемого обычным способом — 1 месяц.   Срок жизни  <b>refresh token</b>, получаемого самостоятельным выписыванием — год.  | |-  > Если мы для вас не завели портфели для ведения торговли в игровом контуре, оставьте заявку на <a href=\"mailto:openapi@alor.ru\">openapi@alor.ru</a> или свяжитесь с нами в [телеграме](https://t.me/AlorOpenAPI).  </br></br> Тестовый контур: `https://apidev.alor.ru`  Боевой контур: `https://api.alor.ru` 
 *
 * OpenAPI spec version: 1.0
 * Contact: openapi@alor.ru
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */
#![allow(unused_imports)]
/// EstimateOrderModel : Модель результата оценки стоимости и количества в лотах

use serde_json::Value;
use rust_decimal::Decimal;
use chrono::{NaiveDateTime, NaiveDate, DateTime, FixedOffset, Utc};

use crate::OutlinePrint;
use crate::models::*;
use crate::date_serializer;
use crate::date_serializer_opt;
use crate::serialize_quoted_numbers;
use crate::serialize_quoted_numbers_opt;
//Uncomment this to deal with limited rfc support on server side
//use crate::datetime_serializer::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EstimateOrderModel {
  #[serde(rename = "commission")]
  commission: Decimal, 
  #[serde(rename = "exchange")]
  exchange: Exchange, 
  #[serde(rename = "notMarginQuantityToBuy")]
  not_margin_quantity_to_buy: Decimal, 
  #[serde(rename = "notMarginQuantityToSell")]
  not_margin_quantity_to_sell: Decimal, 
  #[serde(rename = "orderEvaluation")]
  order_evaluation: Decimal, 
  #[serde(rename = "portfolio")]
  #[serde(default)]
  portfolio: Option<String>, 
  #[serde(rename = "quantityToBuy")]
  quantity_to_buy: Decimal, 
  #[serde(rename = "quantityToSell")]
  quantity_to_sell: Decimal, 
  #[serde(rename = "ticker")]
  #[serde(default)]
  ticker: Option<String> 
}

impl EstimateOrderModel {
  pub fn new(commission: Decimal, exchange: Exchange, not_margin_quantity_to_buy: Decimal, not_margin_quantity_to_sell: Decimal, order_evaluation: Decimal, quantity_to_buy: Decimal, quantity_to_sell: Decimal, ) -> EstimateOrderModel {
    EstimateOrderModel {
      commission: commission,
      exchange: exchange,
      not_margin_quantity_to_buy: not_margin_quantity_to_buy,
      not_margin_quantity_to_sell: not_margin_quantity_to_sell,
      order_evaluation: order_evaluation,
      portfolio: None,
      quantity_to_buy: quantity_to_buy,
      quantity_to_sell: quantity_to_sell,
      ticker: None
    }
  }

  pub fn set_commission(&mut self, commission: Decimal) {
    self.commission = commission;
  }

  pub fn with_commission(mut self, commission: Decimal) -> EstimateOrderModel {
    self.commission = commission;
    self
  }

  pub fn commission(&self) -> &Decimal {
    &self.commission
  }


  pub fn set_exchange(&mut self, exchange: Exchange) {
    self.exchange = exchange;
  }

  pub fn with_exchange(mut self, exchange: Exchange) -> EstimateOrderModel {
    self.exchange = exchange;
    self
  }

  pub fn exchange(&self) -> &Exchange {
    &self.exchange
  }


  pub fn set_not_margin_quantity_to_buy(&mut self, not_margin_quantity_to_buy: Decimal) {
    self.not_margin_quantity_to_buy = not_margin_quantity_to_buy;
  }

  pub fn with_not_margin_quantity_to_buy(mut self, not_margin_quantity_to_buy: Decimal) -> EstimateOrderModel {
    self.not_margin_quantity_to_buy = not_margin_quantity_to_buy;
    self
  }

  pub fn not_margin_quantity_to_buy(&self) -> &Decimal {
    &self.not_margin_quantity_to_buy
  }


  pub fn set_not_margin_quantity_to_sell(&mut self, not_margin_quantity_to_sell: Decimal) {
    self.not_margin_quantity_to_sell = not_margin_quantity_to_sell;
  }

  pub fn with_not_margin_quantity_to_sell(mut self, not_margin_quantity_to_sell: Decimal) -> EstimateOrderModel {
    self.not_margin_quantity_to_sell = not_margin_quantity_to_sell;
    self
  }

  pub fn not_margin_quantity_to_sell(&self) -> &Decimal {
    &self.not_margin_quantity_to_sell
  }


  pub fn set_order_evaluation(&mut self, order_evaluation: Decimal) {
    self.order_evaluation = order_evaluation;
  }

  pub fn with_order_evaluation(mut self, order_evaluation: Decimal) -> EstimateOrderModel {
    self.order_evaluation = order_evaluation;
    self
  }

  pub fn order_evaluation(&self) -> &Decimal {
    &self.order_evaluation
  }


  pub fn set_portfolio(&mut self, portfolio: String) {
    self.portfolio = Some(portfolio);
  }

  pub fn with_portfolio(mut self, portfolio: String) -> EstimateOrderModel {
    self.portfolio = Some(portfolio);
    self
  }

  pub fn portfolio(&self) -> Option<&String> {
    self.portfolio.as_ref()
  }

  pub fn reset_portfolio(&mut self) {
    self.portfolio = None;
  }

  pub fn set_quantity_to_buy(&mut self, quantity_to_buy: Decimal) {
    self.quantity_to_buy = quantity_to_buy;
  }

  pub fn with_quantity_to_buy(mut self, quantity_to_buy: Decimal) -> EstimateOrderModel {
    self.quantity_to_buy = quantity_to_buy;
    self
  }

  pub fn quantity_to_buy(&self) -> &Decimal {
    &self.quantity_to_buy
  }


  pub fn set_quantity_to_sell(&mut self, quantity_to_sell: Decimal) {
    self.quantity_to_sell = quantity_to_sell;
  }

  pub fn with_quantity_to_sell(mut self, quantity_to_sell: Decimal) -> EstimateOrderModel {
    self.quantity_to_sell = quantity_to_sell;
    self
  }

  pub fn quantity_to_sell(&self) -> &Decimal {
    &self.quantity_to_sell
  }


  pub fn set_ticker(&mut self, ticker: String) {
    self.ticker = Some(ticker);
  }

  pub fn with_ticker(mut self, ticker: String) -> EstimateOrderModel {
    self.ticker = Some(ticker);
    self
  }

  pub fn ticker(&self) -> Option<&String> {
    self.ticker.as_ref()
  }

  pub fn reset_ticker(&mut self) {
    self.ticker = None;
  }


  pub fn validate(&self) {
  }

}


