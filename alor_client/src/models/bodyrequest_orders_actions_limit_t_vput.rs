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
pub struct BodyrequestOrdersActionsLimitTVput {
  #[serde(rename = "icebergFixed")]
  ///Видимая постоянная часть айсберг-заявки в лотах
  iceberg_fixed: i32,  // 100 
  #[serde(rename = "icebergVariance")]
  ///Амплитуда отклонения (в % от icebergFixed) случайной надбавки к видимой части айсберг-заявки. Только срочный рынок
  iceberg_variance: Decimal,  // 2 
  #[serde(rename = "id")]
  ///Идентификатор заявки
  id: i64,  // 18936040296 
  #[serde(rename = "instrument")]
  
  instrument: BodyrequestOrdersActionsLimitTvInstrument, 
  #[serde(rename = "price")]
  ///Цена
  price: Decimal,  // 190.97 
  #[serde(rename = "quantity")]
  ///Количество
  quantity: i32,  // 2 
  #[serde(rename = "side")]
  
  side: Side, 
  #[serde(rename = "timeInForce")]
  
  time_in_force: LifePolicy, 
  #[serde(rename = "type")]
  ///Тип заявки
  rtype: String,  // limit 
  #[serde(rename = "user")]
  
  user: BodyrequestOrdersActionsLimitTvUser 
}

impl BodyrequestOrdersActionsLimitTVput {
  pub fn new(iceberg_fixed: i32, iceberg_variance: Decimal, id: i64, instrument: BodyrequestOrdersActionsLimitTvInstrument, price: Decimal, quantity: i32, side: Side, time_in_force: LifePolicy, rtype: String, user: BodyrequestOrdersActionsLimitTvUser, ) -> BodyrequestOrdersActionsLimitTVput {
    BodyrequestOrdersActionsLimitTVput {
      iceberg_fixed: iceberg_fixed,
      iceberg_variance: iceberg_variance,
      id: id,
      instrument: instrument,
      price: price,
      quantity: quantity,
      side: side,
      time_in_force: time_in_force,
      rtype: rtype,
      user: user
    }
  }

  pub fn set_iceberg_fixed(&mut self, iceberg_fixed: i32) {
    self.iceberg_fixed = iceberg_fixed;
  }

  pub fn with_iceberg_fixed(mut self, iceberg_fixed: i32) -> BodyrequestOrdersActionsLimitTVput {
    self.iceberg_fixed = iceberg_fixed;
    self
  }
  ///Видимая постоянная часть айсберг-заявки в лотах
  pub fn iceberg_fixed(&self) -> &i32 {
    &self.iceberg_fixed
  }


  pub fn set_iceberg_variance(&mut self, iceberg_variance: Decimal) {
    self.iceberg_variance = iceberg_variance;
  }

  pub fn with_iceberg_variance(mut self, iceberg_variance: Decimal) -> BodyrequestOrdersActionsLimitTVput {
    self.iceberg_variance = iceberg_variance;
    self
  }
  ///Амплитуда отклонения (в % от icebergFixed) случайной надбавки к видимой части айсберг-заявки. Только срочный рынок
  pub fn iceberg_variance(&self) -> &Decimal {
    &self.iceberg_variance
  }


  pub fn set_id(&mut self, id: i64) {
    self.id = id;
  }

  pub fn with_id(mut self, id: i64) -> BodyrequestOrdersActionsLimitTVput {
    self.id = id;
    self
  }
  ///Идентификатор заявки
  pub fn id(&self) -> &i64 {
    &self.id
  }


  pub fn set_instrument(&mut self, instrument: BodyrequestOrdersActionsLimitTvInstrument) {
    self.instrument = instrument;
  }

  pub fn with_instrument(mut self, instrument: BodyrequestOrdersActionsLimitTvInstrument) -> BodyrequestOrdersActionsLimitTVput {
    self.instrument = instrument;
    self
  }
  
  pub fn instrument(&self) -> &BodyrequestOrdersActionsLimitTvInstrument {
    &self.instrument
  }


  pub fn set_price(&mut self, price: Decimal) {
    self.price = price;
  }

  pub fn with_price(mut self, price: Decimal) -> BodyrequestOrdersActionsLimitTVput {
    self.price = price;
    self
  }
  ///Цена
  pub fn price(&self) -> &Decimal {
    &self.price
  }


  pub fn set_quantity(&mut self, quantity: i32) {
    self.quantity = quantity;
  }

  pub fn with_quantity(mut self, quantity: i32) -> BodyrequestOrdersActionsLimitTVput {
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

  pub fn with_side(mut self, side: Side) -> BodyrequestOrdersActionsLimitTVput {
    self.side = side;
    self
  }
  
  pub fn side(&self) -> &Side {
    &self.side
  }


  pub fn set_time_in_force(&mut self, time_in_force: LifePolicy) {
    self.time_in_force = time_in_force;
  }

  pub fn with_time_in_force(mut self, time_in_force: LifePolicy) -> BodyrequestOrdersActionsLimitTVput {
    self.time_in_force = time_in_force;
    self
  }
  
  pub fn time_in_force(&self) -> &LifePolicy {
    &self.time_in_force
  }


  pub fn set_rtype(&mut self, rtype: String) {
    self.rtype = rtype;
  }

  pub fn with_rtype(mut self, rtype: String) -> BodyrequestOrdersActionsLimitTVput {
    self.rtype = rtype;
    self
  }
  ///Тип заявки
  pub fn rtype(&self) -> &String {
    &self.rtype
  }


  pub fn set_user(&mut self, user: BodyrequestOrdersActionsLimitTvUser) {
    self.user = user;
  }

  pub fn with_user(mut self, user: BodyrequestOrdersActionsLimitTvUser) -> BodyrequestOrdersActionsLimitTVput {
    self.user = user;
    self
  }
  
  pub fn user(&self) -> &BodyrequestOrdersActionsLimitTvUser {
    &self.user
  }



  pub fn validate(&self) {
    let max_value = Decimal::from(20);
    assert!(self.iceberg_variance<=max_value, "BodyrequestOrdersActionsLimitTVput.iceberg_variance should be <= 20");
  }

}


