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
pub struct Order {
  #[serde(rename = "brokerSymbol")]
  broker_symbol: String,  // MOEX:SBER 
  #[serde(rename = "endTime")]
  end_time: String,  // 2020-06-16T23:59:59.9990000 
  #[serde(rename = "exchange")]
  exchange: Exchange, 
  #[serde(rename = "existing")]
  existing: bool,  // true 
  #[serde(rename = "filled")]
  filled: Decimal,  // 1 
  #[serde(rename = "filledQtyBatch")]
  filled_qty_batch: Decimal,  // 1 
  #[serde(rename = "filledQtyUnits")]
  filled_qty_units: Decimal,  // 1 
  #[serde(rename = "id")]
  id: String,  // 18995978560 
  #[serde(rename = "price")]
  price: Decimal,  // 208.6 
  #[serde(rename = "qty")]
  qty: Decimal,  // 1 
  #[serde(rename = "qtyBatch")]
  qty_batch: Decimal,  // 1 
  #[serde(rename = "qtyUnits")]
  qty_units: Decimal,  // 1 
  #[serde(rename = "side")]
  side: Operation, 
  #[serde(rename = "status")]
  status: OrderStatus, 
  #[serde(rename = "symbol")]
  symbol: String,  // SBER 
  #[serde(rename = "transTime")]
  trans_time: String,  // 2020-06-16T23:59:59.9990000 
  #[serde(rename = "type")]
  rtype: OrderType 
}

impl Order {
  pub fn new(broker_symbol: String, end_time: String, exchange: Exchange, existing: bool, filled: Decimal, filled_qty_batch: Decimal, filled_qty_units: Decimal, id: String, price: Decimal, qty: Decimal, qty_batch: Decimal, qty_units: Decimal, side: Operation, status: OrderStatus, symbol: String, trans_time: String, rtype: OrderType, ) -> Order {
    Order {
      broker_symbol: broker_symbol,
      end_time: end_time,
      exchange: exchange,
      existing: existing,
      filled: filled,
      filled_qty_batch: filled_qty_batch,
      filled_qty_units: filled_qty_units,
      id: id,
      price: price,
      qty: qty,
      qty_batch: qty_batch,
      qty_units: qty_units,
      side: side,
      status: status,
      symbol: symbol,
      trans_time: trans_time,
      rtype: rtype
    }
  }

  pub fn set_broker_symbol(&mut self, broker_symbol: String) {
    self.broker_symbol = broker_symbol;
  }

  pub fn with_broker_symbol(mut self, broker_symbol: String) -> Order {
    self.broker_symbol = broker_symbol;
    self
  }

  pub fn broker_symbol(&self) -> &String {
    &self.broker_symbol
  }


  pub fn set_end_time(&mut self, end_time: String) {
    self.end_time = end_time;
  }

  pub fn with_end_time(mut self, end_time: String) -> Order {
    self.end_time = end_time;
    self
  }

  pub fn end_time(&self) -> &String {
    &self.end_time
  }


  pub fn set_exchange(&mut self, exchange: Exchange) {
    self.exchange = exchange;
  }

  pub fn with_exchange(mut self, exchange: Exchange) -> Order {
    self.exchange = exchange;
    self
  }

  pub fn exchange(&self) -> &Exchange {
    &self.exchange
  }


  pub fn set_existing(&mut self, existing: bool) {
    self.existing = existing;
  }

  pub fn with_existing(mut self, existing: bool) -> Order {
    self.existing = existing;
    self
  }

  pub fn existing(&self) -> &bool {
    &self.existing
  }


  pub fn set_filled(&mut self, filled: Decimal) {
    self.filled = filled;
  }

  pub fn with_filled(mut self, filled: Decimal) -> Order {
    self.filled = filled;
    self
  }

  pub fn filled(&self) -> &Decimal {
    &self.filled
  }


  pub fn set_filled_qty_batch(&mut self, filled_qty_batch: Decimal) {
    self.filled_qty_batch = filled_qty_batch;
  }

  pub fn with_filled_qty_batch(mut self, filled_qty_batch: Decimal) -> Order {
    self.filled_qty_batch = filled_qty_batch;
    self
  }

  pub fn filled_qty_batch(&self) -> &Decimal {
    &self.filled_qty_batch
  }


  pub fn set_filled_qty_units(&mut self, filled_qty_units: Decimal) {
    self.filled_qty_units = filled_qty_units;
  }

  pub fn with_filled_qty_units(mut self, filled_qty_units: Decimal) -> Order {
    self.filled_qty_units = filled_qty_units;
    self
  }

  pub fn filled_qty_units(&self) -> &Decimal {
    &self.filled_qty_units
  }


  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }

  pub fn with_id(mut self, id: String) -> Order {
    self.id = id;
    self
  }

  pub fn id(&self) -> &String {
    &self.id
  }


  pub fn set_price(&mut self, price: Decimal) {
    self.price = price;
  }

  pub fn with_price(mut self, price: Decimal) -> Order {
    self.price = price;
    self
  }

  pub fn price(&self) -> &Decimal {
    &self.price
  }


  pub fn set_qty(&mut self, qty: Decimal) {
    self.qty = qty;
  }

  pub fn with_qty(mut self, qty: Decimal) -> Order {
    self.qty = qty;
    self
  }

  pub fn qty(&self) -> &Decimal {
    &self.qty
  }


  pub fn set_qty_batch(&mut self, qty_batch: Decimal) {
    self.qty_batch = qty_batch;
  }

  pub fn with_qty_batch(mut self, qty_batch: Decimal) -> Order {
    self.qty_batch = qty_batch;
    self
  }

  pub fn qty_batch(&self) -> &Decimal {
    &self.qty_batch
  }


  pub fn set_qty_units(&mut self, qty_units: Decimal) {
    self.qty_units = qty_units;
  }

  pub fn with_qty_units(mut self, qty_units: Decimal) -> Order {
    self.qty_units = qty_units;
    self
  }

  pub fn qty_units(&self) -> &Decimal {
    &self.qty_units
  }


  pub fn set_side(&mut self, side: Operation) {
    self.side = side;
  }

  pub fn with_side(mut self, side: Operation) -> Order {
    self.side = side;
    self
  }

  pub fn side(&self) -> &Operation {
    &self.side
  }


  pub fn set_status(&mut self, status: OrderStatus) {
    self.status = status;
  }

  pub fn with_status(mut self, status: OrderStatus) -> Order {
    self.status = status;
    self
  }

  pub fn status(&self) -> &OrderStatus {
    &self.status
  }


  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> Order {
    self.symbol = symbol;
    self
  }

  pub fn symbol(&self) -> &String {
    &self.symbol
  }


  pub fn set_trans_time(&mut self, trans_time: String) {
    self.trans_time = trans_time;
  }

  pub fn with_trans_time(mut self, trans_time: String) -> Order {
    self.trans_time = trans_time;
    self
  }

  pub fn trans_time(&self) -> &String {
    &self.trans_time
  }


  pub fn set_rtype(&mut self, rtype: OrderType) {
    self.rtype = rtype;
  }

  pub fn with_rtype(mut self, rtype: OrderType) -> Order {
    self.rtype = rtype;
    self
  }

  pub fn rtype(&self) -> &OrderType {
    &self.rtype
  }



  pub fn validate(&self) {
  }

}


