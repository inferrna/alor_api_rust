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
/// WsResPositionsGetAndSubscribeData : Сообщение с данными о позициях по ценным бумагам и валютным остаткам

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
pub struct WsResPositionsGetAndSubscribeData {
  #[serde(rename = "avgPrice")]
  ///Средняя цена сделок по позициям
  avg_price: Decimal,  // 82.3 
  #[serde(rename = "brokerSymbol")]
  ///Биржа:Тикер
  broker_symbol: String,  // MOEX:AFLT 
  #[serde(rename = "currentVolume")]
  ///Объём, расчитанный по текущей цене
  current_volume: Decimal,  // 3.879879E+7 
  #[serde(rename = "dailyUnrealisedPl")]
  ///Нереализованная прибыль за день
  daily_unrealised_pl: Decimal,  // 2.8 
  #[serde(rename = "exchange")]
  
  exchange: Exchange, 
  #[serde(rename = "isCurrency")]
  ///True для валютных остатков (денег), false - для торговых инструментов
  is_currency: bool,  // false 
  #[serde(rename = "lotSize")]
  ///Размер лота
  lot_size: Decimal,  // 10.0 
  #[serde(rename = "open")]
  ///Позиции на момент открытия (начала торгов)
  open: i32, 
  #[serde(rename = "openQtyBatch")]
  ///Позиции на момент открытия (начала торгов)
  open_qty_batch: i32, 
  #[serde(rename = "openUnits")]
  ///Позиций на момент открытия (штуки)
  open_units: i32, 
  #[serde(rename = "qty")]
  ///Количество (лоты)
  qty: i32, 
  #[serde(rename = "qtyBatch")]
  ///Количество (лоты)
  qty_batch: i32, 
  #[serde(rename = "qtyT0")]
  ///Количество на дату \"Т0\" (штуки)
  qty_t0: i32, 
  #[serde(rename = "qtyT0Batch")]
  ///Количество на дату \"Т0\" (лоты)
  qty_t0_batch: i32, 
  #[serde(rename = "qtyT1")]
  ///Количество на дату \"Т1\" (штуки)
  qty_t1: i32, 
  #[serde(rename = "qtyT1Batch")]
  ///Количество на дату \"Т1\" (лоты)
  qty_t1_batch: i32, 
  #[serde(rename = "qtyT2")]
  ///Количество на дату \"Т2\" (штуки)
  qty_t2: i32, 
  #[serde(rename = "qtyT2Batch")]
  ///Количество на дату \"Т2\" (лоты)
  qty_t2_batch: i32, 
  #[serde(rename = "qtyTFuture")]
  ///Количество на дату \"TFuture\" (штуки)
  qty_t_future: i32, 
  #[serde(rename = "qtyTFutureBatch")]
  ///Количество на дату \"TFuture\" (лоты)
  qty_t_future_batch: i32, 
  #[serde(rename = "qtyUnits")]
  ///Количество (штуки)
  qty_units: i32, 
  #[serde(rename = "shortName")]
  ///Наименование инструмента
  short_name: String,  // Аэрофлот 
  #[serde(rename = "symbol")]
  ///Тикер (Код финансового инструмента)
  symbol: String,  // AFLT 
  #[serde(rename = "unrealisedPl")]
  ///Нереализованная прибыль
  unrealised_pl: Decimal,  // 2.8 
  #[serde(rename = "volume")]
  ///Объём, расчитанный по средней цене
  volume: Decimal  // 3.876708E+7 
}

impl WsResPositionsGetAndSubscribeData {
  pub fn new(avg_price: Decimal, broker_symbol: String, current_volume: Decimal, daily_unrealised_pl: Decimal, exchange: Exchange, is_currency: bool, lot_size: Decimal, open: i32, open_qty_batch: i32, open_units: i32, qty: i32, qty_batch: i32, qty_t0: i32, qty_t0_batch: i32, qty_t1: i32, qty_t1_batch: i32, qty_t2: i32, qty_t2_batch: i32, qty_t_future: i32, qty_t_future_batch: i32, qty_units: i32, short_name: String, symbol: String, unrealised_pl: Decimal, volume: Decimal, ) -> WsResPositionsGetAndSubscribeData {
    WsResPositionsGetAndSubscribeData {
      avg_price: avg_price,
      broker_symbol: broker_symbol,
      current_volume: current_volume,
      daily_unrealised_pl: daily_unrealised_pl,
      exchange: exchange,
      is_currency: is_currency,
      lot_size: lot_size,
      open: open,
      open_qty_batch: open_qty_batch,
      open_units: open_units,
      qty: qty,
      qty_batch: qty_batch,
      qty_t0: qty_t0,
      qty_t0_batch: qty_t0_batch,
      qty_t1: qty_t1,
      qty_t1_batch: qty_t1_batch,
      qty_t2: qty_t2,
      qty_t2_batch: qty_t2_batch,
      qty_t_future: qty_t_future,
      qty_t_future_batch: qty_t_future_batch,
      qty_units: qty_units,
      short_name: short_name,
      symbol: symbol,
      unrealised_pl: unrealised_pl,
      volume: volume
    }
  }

  pub fn set_avg_price(&mut self, avg_price: Decimal) {
    self.avg_price = avg_price;
  }

  pub fn with_avg_price(mut self, avg_price: Decimal) -> WsResPositionsGetAndSubscribeData {
    self.avg_price = avg_price;
    self
  }
  ///Средняя цена сделок по позициям
  pub fn avg_price(&self) -> &Decimal {
    &self.avg_price
  }


  pub fn set_broker_symbol(&mut self, broker_symbol: String) {
    self.broker_symbol = broker_symbol;
  }

  pub fn with_broker_symbol(mut self, broker_symbol: String) -> WsResPositionsGetAndSubscribeData {
    self.broker_symbol = broker_symbol;
    self
  }
  ///Биржа:Тикер
  pub fn broker_symbol(&self) -> &String {
    &self.broker_symbol
  }


  pub fn set_current_volume(&mut self, current_volume: Decimal) {
    self.current_volume = current_volume;
  }

  pub fn with_current_volume(mut self, current_volume: Decimal) -> WsResPositionsGetAndSubscribeData {
    self.current_volume = current_volume;
    self
  }
  ///Объём, расчитанный по текущей цене
  pub fn current_volume(&self) -> &Decimal {
    &self.current_volume
  }


  pub fn set_daily_unrealised_pl(&mut self, daily_unrealised_pl: Decimal) {
    self.daily_unrealised_pl = daily_unrealised_pl;
  }

  pub fn with_daily_unrealised_pl(mut self, daily_unrealised_pl: Decimal) -> WsResPositionsGetAndSubscribeData {
    self.daily_unrealised_pl = daily_unrealised_pl;
    self
  }
  ///Нереализованная прибыль за день
  pub fn daily_unrealised_pl(&self) -> &Decimal {
    &self.daily_unrealised_pl
  }


  pub fn set_exchange(&mut self, exchange: Exchange) {
    self.exchange = exchange;
  }

  pub fn with_exchange(mut self, exchange: Exchange) -> WsResPositionsGetAndSubscribeData {
    self.exchange = exchange;
    self
  }
  
  pub fn exchange(&self) -> &Exchange {
    &self.exchange
  }


  pub fn set_is_currency(&mut self, is_currency: bool) {
    self.is_currency = is_currency;
  }

  pub fn with_is_currency(mut self, is_currency: bool) -> WsResPositionsGetAndSubscribeData {
    self.is_currency = is_currency;
    self
  }
  ///True для валютных остатков (денег), false - для торговых инструментов
  pub fn is_currency(&self) -> &bool {
    &self.is_currency
  }


  pub fn set_lot_size(&mut self, lot_size: Decimal) {
    self.lot_size = lot_size;
  }

  pub fn with_lot_size(mut self, lot_size: Decimal) -> WsResPositionsGetAndSubscribeData {
    self.lot_size = lot_size;
    self
  }
  ///Размер лота
  pub fn lot_size(&self) -> &Decimal {
    &self.lot_size
  }


  pub fn set_open(&mut self, open: i32) {
    self.open = open;
  }

  pub fn with_open(mut self, open: i32) -> WsResPositionsGetAndSubscribeData {
    self.open = open;
    self
  }
  ///Позиции на момент открытия (начала торгов)
  pub fn open(&self) -> &i32 {
    &self.open
  }


  pub fn set_open_qty_batch(&mut self, open_qty_batch: i32) {
    self.open_qty_batch = open_qty_batch;
  }

  pub fn with_open_qty_batch(mut self, open_qty_batch: i32) -> WsResPositionsGetAndSubscribeData {
    self.open_qty_batch = open_qty_batch;
    self
  }
  ///Позиции на момент открытия (начала торгов)
  pub fn open_qty_batch(&self) -> &i32 {
    &self.open_qty_batch
  }


  pub fn set_open_units(&mut self, open_units: i32) {
    self.open_units = open_units;
  }

  pub fn with_open_units(mut self, open_units: i32) -> WsResPositionsGetAndSubscribeData {
    self.open_units = open_units;
    self
  }
  ///Позиций на момент открытия (штуки)
  pub fn open_units(&self) -> &i32 {
    &self.open_units
  }


  pub fn set_qty(&mut self, qty: i32) {
    self.qty = qty;
  }

  pub fn with_qty(mut self, qty: i32) -> WsResPositionsGetAndSubscribeData {
    self.qty = qty;
    self
  }
  ///Количество (лоты)
  pub fn qty(&self) -> &i32 {
    &self.qty
  }


  pub fn set_qty_batch(&mut self, qty_batch: i32) {
    self.qty_batch = qty_batch;
  }

  pub fn with_qty_batch(mut self, qty_batch: i32) -> WsResPositionsGetAndSubscribeData {
    self.qty_batch = qty_batch;
    self
  }
  ///Количество (лоты)
  pub fn qty_batch(&self) -> &i32 {
    &self.qty_batch
  }


  pub fn set_qty_t0(&mut self, qty_t0: i32) {
    self.qty_t0 = qty_t0;
  }

  pub fn with_qty_t0(mut self, qty_t0: i32) -> WsResPositionsGetAndSubscribeData {
    self.qty_t0 = qty_t0;
    self
  }
  ///Количество на дату \"Т0\" (штуки)
  pub fn qty_t0(&self) -> &i32 {
    &self.qty_t0
  }


  pub fn set_qty_t0_batch(&mut self, qty_t0_batch: i32) {
    self.qty_t0_batch = qty_t0_batch;
  }

  pub fn with_qty_t0_batch(mut self, qty_t0_batch: i32) -> WsResPositionsGetAndSubscribeData {
    self.qty_t0_batch = qty_t0_batch;
    self
  }
  ///Количество на дату \"Т0\" (лоты)
  pub fn qty_t0_batch(&self) -> &i32 {
    &self.qty_t0_batch
  }


  pub fn set_qty_t1(&mut self, qty_t1: i32) {
    self.qty_t1 = qty_t1;
  }

  pub fn with_qty_t1(mut self, qty_t1: i32) -> WsResPositionsGetAndSubscribeData {
    self.qty_t1 = qty_t1;
    self
  }
  ///Количество на дату \"Т1\" (штуки)
  pub fn qty_t1(&self) -> &i32 {
    &self.qty_t1
  }


  pub fn set_qty_t1_batch(&mut self, qty_t1_batch: i32) {
    self.qty_t1_batch = qty_t1_batch;
  }

  pub fn with_qty_t1_batch(mut self, qty_t1_batch: i32) -> WsResPositionsGetAndSubscribeData {
    self.qty_t1_batch = qty_t1_batch;
    self
  }
  ///Количество на дату \"Т1\" (лоты)
  pub fn qty_t1_batch(&self) -> &i32 {
    &self.qty_t1_batch
  }


  pub fn set_qty_t2(&mut self, qty_t2: i32) {
    self.qty_t2 = qty_t2;
  }

  pub fn with_qty_t2(mut self, qty_t2: i32) -> WsResPositionsGetAndSubscribeData {
    self.qty_t2 = qty_t2;
    self
  }
  ///Количество на дату \"Т2\" (штуки)
  pub fn qty_t2(&self) -> &i32 {
    &self.qty_t2
  }


  pub fn set_qty_t2_batch(&mut self, qty_t2_batch: i32) {
    self.qty_t2_batch = qty_t2_batch;
  }

  pub fn with_qty_t2_batch(mut self, qty_t2_batch: i32) -> WsResPositionsGetAndSubscribeData {
    self.qty_t2_batch = qty_t2_batch;
    self
  }
  ///Количество на дату \"Т2\" (лоты)
  pub fn qty_t2_batch(&self) -> &i32 {
    &self.qty_t2_batch
  }


  pub fn set_qty_t_future(&mut self, qty_t_future: i32) {
    self.qty_t_future = qty_t_future;
  }

  pub fn with_qty_t_future(mut self, qty_t_future: i32) -> WsResPositionsGetAndSubscribeData {
    self.qty_t_future = qty_t_future;
    self
  }
  ///Количество на дату \"TFuture\" (штуки)
  pub fn qty_t_future(&self) -> &i32 {
    &self.qty_t_future
  }


  pub fn set_qty_t_future_batch(&mut self, qty_t_future_batch: i32) {
    self.qty_t_future_batch = qty_t_future_batch;
  }

  pub fn with_qty_t_future_batch(mut self, qty_t_future_batch: i32) -> WsResPositionsGetAndSubscribeData {
    self.qty_t_future_batch = qty_t_future_batch;
    self
  }
  ///Количество на дату \"TFuture\" (лоты)
  pub fn qty_t_future_batch(&self) -> &i32 {
    &self.qty_t_future_batch
  }


  pub fn set_qty_units(&mut self, qty_units: i32) {
    self.qty_units = qty_units;
  }

  pub fn with_qty_units(mut self, qty_units: i32) -> WsResPositionsGetAndSubscribeData {
    self.qty_units = qty_units;
    self
  }
  ///Количество (штуки)
  pub fn qty_units(&self) -> &i32 {
    &self.qty_units
  }


  pub fn set_short_name(&mut self, short_name: String) {
    self.short_name = short_name;
  }

  pub fn with_short_name(mut self, short_name: String) -> WsResPositionsGetAndSubscribeData {
    self.short_name = short_name;
    self
  }
  ///Наименование инструмента
  pub fn short_name(&self) -> &String {
    &self.short_name
  }


  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> WsResPositionsGetAndSubscribeData {
    self.symbol = symbol;
    self
  }
  ///Тикер (Код финансового инструмента)
  pub fn symbol(&self) -> &String {
    &self.symbol
  }


  pub fn set_unrealised_pl(&mut self, unrealised_pl: Decimal) {
    self.unrealised_pl = unrealised_pl;
  }

  pub fn with_unrealised_pl(mut self, unrealised_pl: Decimal) -> WsResPositionsGetAndSubscribeData {
    self.unrealised_pl = unrealised_pl;
    self
  }
  ///Нереализованная прибыль
  pub fn unrealised_pl(&self) -> &Decimal {
    &self.unrealised_pl
  }


  pub fn set_volume(&mut self, volume: Decimal) {
    self.volume = volume;
  }

  pub fn with_volume(mut self, volume: Decimal) -> WsResPositionsGetAndSubscribeData {
    self.volume = volume;
    self
  }
  ///Объём, расчитанный по средней цене
  pub fn volume(&self) -> &Decimal {
    &self.volume
  }



  pub fn validate(&self) {
  }

}


