/* 
 * Alor OpenAPI V2
 *
 * API для работы с торговой системой АЛОР Брокер. Предоставляет интерфейсы для выставления заявок и получения биржевой информации.  Данные для неавторизованных запросов предоставляются с задержкой от 15 минут, для авторизованных - без задержек.   Публичная биржевая информация может быть получена через HTTP(S) API, а также доступна через однократно установленное WebSocket соединение. <br> **Внимание!** WebSocket соединения могут и будут разрываться *(например, если клиент не успевает обрабатывать сообщения и на стороне API в буфере накопится более 5000 событий)*. <br> Во внешнем ПО необходимо предусмотреть механизмы переподключения и переподписки (при необходимости)! <br> <br>  В OpenAPI V2 доступны \"Московская Биржа\" (MOEX) и \"Биржа СПБ\" (SPBX).   <h4> Доступные типы данных </h4>  * Все сделки  * Все заявки  * Информация по инструментам  * Котировки  * Биржевые стаканы  * Исторические данные  * Позиции  * Информация о клиенте  <h4>Поддерживаемые виды заявок</h4>  * рыночные  * лимитные  * стоп-лосс  * тейк-профит  * стоп-лосс лимит  * тейк-профит лимит  <h4>    Ограничения по частоте запросов     </h4> <p>На текущий момент ограничений по количеству запросов в минуту нет, однако есть ограничение на общее количество подписок (сотни тысяч). При достижении лимита подписок клиент будет заблокирован и в течение нескольких минут не сможет создавать новые подписки. Уже существующие подписки продолжат работать. <br/>  Сервер может обрабатывать \"тяжелые\" запросы (пример - история за все время) и запросы без авторизации с меньшим приоритетом.<br/> </p>  <h4>Получение списка портфелей</h4> <p>Получить список доступных портфелей можно из JWT токена</p> <p>Для получения списка доступных портфелей необходимо декодировать JWT токен. Портфели находятся в поле <b>portfolios</b>.</p> <br/>  <h2> Авторизация </h2>  <h4>OAuth</h4>  <b>Внимание!</b>   JWT и refresh token — равносильны логину и паролю. Их нужно скрывать от публичного доступа.  <h4>Для разработчиков сторонних приложений, в которых торговлю будут вести их пользователи.</h4>  Мы предоставляем сервис для авторизации сторониих приложений по стандарту OAuth 2.0. С примером приложения, использующего OAuth сервис для авторизации пользователей можно ознакомиться в разделе  <a href=\"/examples\">Примеры</a>.  Список разрешений (scopes), которые могут быть выданы приложению: <table>   <tr>     <td><b>OrdersRead</b></td>     <td>Чтение выставленных заявок</td>   </tr>   <tr>     <td><b>OrdersCreate</b></td>     <td>Выставление заявок</td>   </tr>   <tr>     <td><b>Trades</b></td>     <td>Чтение совершенных сделок</td>   </tr>   <tr>     <td><b>Personal</b></td>     <td>Персональная информация: ФИО, почта и т.п.</td>   </tr>   <tr>     <td><b>Stats</b></td>     <td>Статистика: прибыль, средние цены и т.п.</td>   </tr> </table>  <h4>Для ведения операций от своего имени</h4>  Выписать себе <b>refresh token</b> для ведения операций от своего имени [можно здесь](https://alor.dev/open-api-tokens).  <h2>Краткое описание работы с авторизацией</h2>  Чтобы выполнить авторизованный запрос, добавьте в запрос заголовок с именем \"Authorization\" и значением, состоящим из префикса `\"Bearer \"` и валидного JWT токена.  Срок жизни JWT короткий: это сделано для безопасности.  Для большинства вариантов использования API мы рекоммендуем использовать механизм  <b>refresh token</b> .  Механизм  <b>refresh token</b>  позволяет получать JWT с новым сроком жизни. Для этого отправьте POST запрос на адрес `https://oauthdev.alor.ru/refresh?token={refreshToken}` *(тестовый контур)* или `https://oauth.alor.ru/refresh?token={refreshToken}` *(боевой контур)*. Если у  <b>refresh token</b>  не истек срок жизни и не он не был отозван, то в теле ответа в поле AccessToken вернётся свежий JWT токен.   Срок жизни  <b>refresh token</b>, получаемого обычным способом — 1 месяц.   Срок жизни  <b>refresh token</b>, получаемого самостоятельным выписыванием — год.  | |-  > Если мы для вас не завели портфели для ведения торговли в игровом контуре, оставьте заявку на <a href=\"mailto:openapi@alor.ru\">openapi@alor.ru</a> или свяжитесь с нами в [телеграме](https://t.me/AlorOpenAPI).  </br></br> Тестовый контур: `https://apidev.alor.ru`  Боевой контур: `https://api.alor.ru` 
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
pub struct WsResQuotesSubscribeData {
  #[serde(rename = "accruedInt")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(default)]
  ///Начислено (НКД)
  accrued_int: Option<Decimal>,  // 0 
  #[serde(rename = "accrued_interest")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(default)]
  ///Начислено (НКД)
  accrued_interest: Option<Decimal>,  // 0 
  #[serde(rename = "ask")]
  ///Аск
  ask: Decimal,  // 303.65 
  #[serde(rename = "bid")]
  ///Бид
  bid: Decimal,  // 303.59 
  #[serde(rename = "change")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(default)]
  ///Разность цены и цены предыдущего закрытия
  change: Option<Decimal>,  // -0.11 
  #[serde(rename = "change_percent")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(default)]
  ///Относительное изменение цены
  change_percent: Option<Decimal>,  // -0.04 
  #[serde(rename = "description")]
  ///Короткое описание на русском языке
  description: String,  // Сбербанк России ПАО ао 
  #[serde(rename = "exchange")]
  
  exchange: Exchange, 
  #[serde(rename = "facevalue")]
  
  facevalue: Decimal,  // 3 
  #[serde(rename = "high_price")]
  ///Максимальная цена
  high_price: Decimal,  // 305 
  #[serde(rename = "last_price")]
  ///Последняя цена
  last_price: Decimal,  // 303.59 
  #[serde(rename = "last_price_timestamp")]
  ///Время последней цены (UTC)
  last_price_timestamp: i64,  // 1620221538 
  #[serde(rename = "lotsize")]
  ///Размер лота
  lotsize: Decimal,  // 10.0 
  #[serde(rename = "lotvalue")]
  
  lotvalue: Decimal,  // 3035.9 
  #[serde(rename = "low_price")]
  ///Минимальная цена
  low_price: Decimal,  // 302.71 
  #[serde(rename = "open_price")]
  ///Цена открытия
  open_price: Decimal,  // 304.01 
  #[serde(rename = "prev_close_price")]
  ///Цена предыдущего закрытия
  prev_close_price: Decimal,  // 303.7 
  #[serde(rename = "symbol")]
  ///Тикер (Код финансового инструмента)
  symbol: String,  // SBER 
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(default)]
  
  rtype: Option<String>,  // CS 
  #[serde(rename = "volume")]
  ///Объём
  volume: Decimal,  // 3.876708E+7 
  #[serde(rename = "yield")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(default)]
  
  ryield: Option<i32> 
}

impl WsResQuotesSubscribeData {
  pub fn new(ask: Decimal, bid: Decimal, description: String, exchange: Exchange, facevalue: Decimal, high_price: Decimal, last_price: Decimal, last_price_timestamp: i64, lotsize: Decimal, lotvalue: Decimal, low_price: Decimal, open_price: Decimal, prev_close_price: Decimal, symbol: String, volume: Decimal, ) -> WsResQuotesSubscribeData {
    WsResQuotesSubscribeData {
      accrued_int: None,
      accrued_interest: None,
      ask: ask,
      bid: bid,
      change: None,
      change_percent: None,
      description: description,
      exchange: exchange,
      facevalue: facevalue,
      high_price: high_price,
      last_price: last_price,
      last_price_timestamp: last_price_timestamp,
      lotsize: lotsize,
      lotvalue: lotvalue,
      low_price: low_price,
      open_price: open_price,
      prev_close_price: prev_close_price,
      symbol: symbol,
      rtype: None,
      volume: volume,
      ryield: None
    }
  }

  pub fn set_accrued_int(&mut self, accrued_int: Decimal) {
    self.accrued_int = Some(accrued_int);
  }

  pub fn with_accrued_int(mut self, accrued_int: Decimal) -> WsResQuotesSubscribeData {
    self.accrued_int = Some(accrued_int);
    self
  }
  ///Начислено (НКД)
  pub fn accrued_int(&self) -> Option<&Decimal> {
    self.accrued_int.as_ref()
  }

  pub fn reset_accrued_int(&mut self) {
    self.accrued_int = None;
  }

  pub fn set_accrued_interest(&mut self, accrued_interest: Decimal) {
    self.accrued_interest = Some(accrued_interest);
  }

  pub fn with_accrued_interest(mut self, accrued_interest: Decimal) -> WsResQuotesSubscribeData {
    self.accrued_interest = Some(accrued_interest);
    self
  }
  ///Начислено (НКД)
  pub fn accrued_interest(&self) -> Option<&Decimal> {
    self.accrued_interest.as_ref()
  }

  pub fn reset_accrued_interest(&mut self) {
    self.accrued_interest = None;
  }

  pub fn set_ask(&mut self, ask: Decimal) {
    self.ask = ask;
  }

  pub fn with_ask(mut self, ask: Decimal) -> WsResQuotesSubscribeData {
    self.ask = ask;
    self
  }
  ///Аск
  pub fn ask(&self) -> &Decimal {
    &self.ask
  }


  pub fn set_bid(&mut self, bid: Decimal) {
    self.bid = bid;
  }

  pub fn with_bid(mut self, bid: Decimal) -> WsResQuotesSubscribeData {
    self.bid = bid;
    self
  }
  ///Бид
  pub fn bid(&self) -> &Decimal {
    &self.bid
  }


  pub fn set_change(&mut self, change: Decimal) {
    self.change = Some(change);
  }

  pub fn with_change(mut self, change: Decimal) -> WsResQuotesSubscribeData {
    self.change = Some(change);
    self
  }
  ///Разность цены и цены предыдущего закрытия
  pub fn change(&self) -> Option<&Decimal> {
    self.change.as_ref()
  }

  pub fn reset_change(&mut self) {
    self.change = None;
  }

  pub fn set_change_percent(&mut self, change_percent: Decimal) {
    self.change_percent = Some(change_percent);
  }

  pub fn with_change_percent(mut self, change_percent: Decimal) -> WsResQuotesSubscribeData {
    self.change_percent = Some(change_percent);
    self
  }
  ///Относительное изменение цены
  pub fn change_percent(&self) -> Option<&Decimal> {
    self.change_percent.as_ref()
  }

  pub fn reset_change_percent(&mut self) {
    self.change_percent = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = description;
  }

  pub fn with_description(mut self, description: String) -> WsResQuotesSubscribeData {
    self.description = description;
    self
  }
  ///Короткое описание на русском языке
  pub fn description(&self) -> &String {
    &self.description
  }


  pub fn set_exchange(&mut self, exchange: Exchange) {
    self.exchange = exchange;
  }

  pub fn with_exchange(mut self, exchange: Exchange) -> WsResQuotesSubscribeData {
    self.exchange = exchange;
    self
  }
  
  pub fn exchange(&self) -> &Exchange {
    &self.exchange
  }


  pub fn set_facevalue(&mut self, facevalue: Decimal) {
    self.facevalue = facevalue;
  }

  pub fn with_facevalue(mut self, facevalue: Decimal) -> WsResQuotesSubscribeData {
    self.facevalue = facevalue;
    self
  }
  
  pub fn facevalue(&self) -> &Decimal {
    &self.facevalue
  }


  pub fn set_high_price(&mut self, high_price: Decimal) {
    self.high_price = high_price;
  }

  pub fn with_high_price(mut self, high_price: Decimal) -> WsResQuotesSubscribeData {
    self.high_price = high_price;
    self
  }
  ///Максимальная цена
  pub fn high_price(&self) -> &Decimal {
    &self.high_price
  }


  pub fn set_last_price(&mut self, last_price: Decimal) {
    self.last_price = last_price;
  }

  pub fn with_last_price(mut self, last_price: Decimal) -> WsResQuotesSubscribeData {
    self.last_price = last_price;
    self
  }
  ///Последняя цена
  pub fn last_price(&self) -> &Decimal {
    &self.last_price
  }


  pub fn set_last_price_timestamp(&mut self, last_price_timestamp: i64) {
    self.last_price_timestamp = last_price_timestamp;
  }

  pub fn with_last_price_timestamp(mut self, last_price_timestamp: i64) -> WsResQuotesSubscribeData {
    self.last_price_timestamp = last_price_timestamp;
    self
  }
  ///Время последней цены (UTC)
  pub fn last_price_timestamp(&self) -> &i64 {
    &self.last_price_timestamp
  }


  pub fn set_lotsize(&mut self, lotsize: Decimal) {
    self.lotsize = lotsize;
  }

  pub fn with_lotsize(mut self, lotsize: Decimal) -> WsResQuotesSubscribeData {
    self.lotsize = lotsize;
    self
  }
  ///Размер лота
  pub fn lotsize(&self) -> &Decimal {
    &self.lotsize
  }


  pub fn set_lotvalue(&mut self, lotvalue: Decimal) {
    self.lotvalue = lotvalue;
  }

  pub fn with_lotvalue(mut self, lotvalue: Decimal) -> WsResQuotesSubscribeData {
    self.lotvalue = lotvalue;
    self
  }
  
  pub fn lotvalue(&self) -> &Decimal {
    &self.lotvalue
  }


  pub fn set_low_price(&mut self, low_price: Decimal) {
    self.low_price = low_price;
  }

  pub fn with_low_price(mut self, low_price: Decimal) -> WsResQuotesSubscribeData {
    self.low_price = low_price;
    self
  }
  ///Минимальная цена
  pub fn low_price(&self) -> &Decimal {
    &self.low_price
  }


  pub fn set_open_price(&mut self, open_price: Decimal) {
    self.open_price = open_price;
  }

  pub fn with_open_price(mut self, open_price: Decimal) -> WsResQuotesSubscribeData {
    self.open_price = open_price;
    self
  }
  ///Цена открытия
  pub fn open_price(&self) -> &Decimal {
    &self.open_price
  }


  pub fn set_prev_close_price(&mut self, prev_close_price: Decimal) {
    self.prev_close_price = prev_close_price;
  }

  pub fn with_prev_close_price(mut self, prev_close_price: Decimal) -> WsResQuotesSubscribeData {
    self.prev_close_price = prev_close_price;
    self
  }
  ///Цена предыдущего закрытия
  pub fn prev_close_price(&self) -> &Decimal {
    &self.prev_close_price
  }


  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> WsResQuotesSubscribeData {
    self.symbol = symbol;
    self
  }
  ///Тикер (Код финансового инструмента)
  pub fn symbol(&self) -> &String {
    &self.symbol
  }


  pub fn set_rtype(&mut self, rtype: String) {
    self.rtype = Some(rtype);
  }

  pub fn with_rtype(mut self, rtype: String) -> WsResQuotesSubscribeData {
    self.rtype = Some(rtype);
    self
  }
  
  pub fn rtype(&self) -> Option<&String> {
    self.rtype.as_ref()
  }

  pub fn reset_rtype(&mut self) {
    self.rtype = None;
  }

  pub fn set_volume(&mut self, volume: Decimal) {
    self.volume = volume;
  }

  pub fn with_volume(mut self, volume: Decimal) -> WsResQuotesSubscribeData {
    self.volume = volume;
    self
  }
  ///Объём
  pub fn volume(&self) -> &Decimal {
    &self.volume
  }


  pub fn set_ryield(&mut self, ryield: i32) {
    self.ryield = Some(ryield);
  }

  pub fn with_ryield(mut self, ryield: i32) -> WsResQuotesSubscribeData {
    self.ryield = Some(ryield);
    self
  }
  
  pub fn ryield(&self) -> Option<&i32> {
    self.ryield.as_ref()
  }

  pub fn reset_ryield(&mut self) {
    self.ryield = None;
  }


  pub fn validate(&self) {
  }

}


