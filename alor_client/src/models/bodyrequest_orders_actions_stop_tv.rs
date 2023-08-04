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
pub struct BodyrequestOrdersActionsStopTv {
  #[serde(rename = "endTime")]
  //Uncomment this also to deal with limited rfc support on server side
  //#[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
  ///Срок действия
  end_time: DateTime<Utc>,  // 2020-05-21T20:59Z 
  #[serde(rename = "instrument")]
  
  instrument: BodyrequestOrdersActionsLimitTvInstrument, 
  #[serde(rename = "quantity")]
  ///Количество
  quantity: i32,  // 2 
  #[serde(rename = "side")]
  
  side: Side, 
  #[serde(rename = "stopEndUnixTime")]
  ///Срок действия (UTC) в формате Unix Time seconds
  stop_end_unix_time: i64,  // 1590094740 
  #[serde(rename = "symbol")]
  ///Тикер (Код финансового инструмента)
  symbol: String,  // SBER 
  #[serde(rename = "takeProfit")]
  ///Стоп-цена
  take_profit: Decimal,  // 190.98 
  #[serde(rename = "triggerPrice")]
  ///Стоп-цена
  trigger_price: Decimal,  // 190.98 
  #[serde(rename = "type")]
  ///Тип заявки
  rtype: String,  // takeprofit 
  #[serde(rename = "user")]
  
  user: BodyrequestOrdersActionsMarketTVputUser 
}

impl BodyrequestOrdersActionsStopTv {
  pub fn new(end_time: DateTime<Utc>, instrument: BodyrequestOrdersActionsLimitTvInstrument, quantity: i32, side: Side, stop_end_unix_time: i64, symbol: String, take_profit: Decimal, trigger_price: Decimal, rtype: String, user: BodyrequestOrdersActionsMarketTVputUser, ) -> BodyrequestOrdersActionsStopTv {
    BodyrequestOrdersActionsStopTv {
      end_time: end_time,
      instrument: instrument,
      quantity: quantity,
      side: side,
      stop_end_unix_time: stop_end_unix_time,
      symbol: symbol,
      take_profit: take_profit,
      trigger_price: trigger_price,
      rtype: rtype,
      user: user
    }
  }

  pub fn set_end_time(&mut self, end_time: DateTime<Utc>) {
    self.end_time = end_time;
  }

  pub fn with_end_time(mut self, end_time: DateTime<Utc>) -> BodyrequestOrdersActionsStopTv {
    self.end_time = end_time;
    self
  }
  ///Срок действия
  pub fn end_time(&self) -> &DateTime<Utc> {
    &self.end_time
  }


  pub fn set_instrument(&mut self, instrument: BodyrequestOrdersActionsLimitTvInstrument) {
    self.instrument = instrument;
  }

  pub fn with_instrument(mut self, instrument: BodyrequestOrdersActionsLimitTvInstrument) -> BodyrequestOrdersActionsStopTv {
    self.instrument = instrument;
    self
  }
  
  pub fn instrument(&self) -> &BodyrequestOrdersActionsLimitTvInstrument {
    &self.instrument
  }


  pub fn set_quantity(&mut self, quantity: i32) {
    self.quantity = quantity;
  }

  pub fn with_quantity(mut self, quantity: i32) -> BodyrequestOrdersActionsStopTv {
    self.quantity = quantity;
    self
  }
  ///Количество
  pub fn quantity(&self) -> &i32 {
    &self.quantity
  }


  pub fn set_side(&mut self, side: Side) {
    self.side = side;
  }

  pub fn with_side(mut self, side: Side) -> BodyrequestOrdersActionsStopTv {
    self.side = side;
    self
  }
  
  pub fn side(&self) -> &Side {
    &self.side
  }


  pub fn set_stop_end_unix_time(&mut self, stop_end_unix_time: i64) {
    self.stop_end_unix_time = stop_end_unix_time;
  }

  pub fn with_stop_end_unix_time(mut self, stop_end_unix_time: i64) -> BodyrequestOrdersActionsStopTv {
    self.stop_end_unix_time = stop_end_unix_time;
    self
  }
  ///Срок действия (UTC) в формате Unix Time seconds
  pub fn stop_end_unix_time(&self) -> &i64 {
    &self.stop_end_unix_time
  }


  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> BodyrequestOrdersActionsStopTv {
    self.symbol = symbol;
    self
  }
  ///Тикер (Код финансового инструмента)
  pub fn symbol(&self) -> &String {
    &self.symbol
  }


  pub fn set_take_profit(&mut self, take_profit: Decimal) {
    self.take_profit = take_profit;
  }

  pub fn with_take_profit(mut self, take_profit: Decimal) -> BodyrequestOrdersActionsStopTv {
    self.take_profit = take_profit;
    self
  }
  ///Стоп-цена
  pub fn take_profit(&self) -> &Decimal {
    &self.take_profit
  }


  pub fn set_trigger_price(&mut self, trigger_price: Decimal) {
    self.trigger_price = trigger_price;
  }

  pub fn with_trigger_price(mut self, trigger_price: Decimal) -> BodyrequestOrdersActionsStopTv {
    self.trigger_price = trigger_price;
    self
  }
  ///Стоп-цена
  pub fn trigger_price(&self) -> &Decimal {
    &self.trigger_price
  }


  pub fn set_rtype(&mut self, rtype: String) {
    self.rtype = rtype;
  }

  pub fn with_rtype(mut self, rtype: String) -> BodyrequestOrdersActionsStopTv {
    self.rtype = rtype;
    self
  }
  ///Тип заявки
  pub fn rtype(&self) -> &String {
    &self.rtype
  }


  pub fn set_user(&mut self, user: BodyrequestOrdersActionsMarketTVputUser) {
    self.user = user;
  }

  pub fn with_user(mut self, user: BodyrequestOrdersActionsMarketTVputUser) -> BodyrequestOrdersActionsStopTv {
    self.user = user;
    self
  }
  
  pub fn user(&self) -> &BodyrequestOrdersActionsMarketTVputUser {
    &self.user
  }



  pub fn validate(&self) {
  }

}


