/* 
 * Alor OpenAPI V2
 *
 * API для работы с торговой системой АЛОР Брокер. Предоставляет интерфейсы для выставления заявок и получения биржевой информации.  Данные для неавторизованных запросов предоставляются с задержкой от 15 минут, для авторизованных - без задержек.   Публичная биржевая информация может быть получена через HTTP(S) API, а также доступна через однократно установленное WebSocket соединение.  **Внимание!** WebSocket соединения могут и будут разрываться *(например, если клиент не успевает обрабатывать сообщения и на стороне API в буфере накопится более 5000 событий)*.  Во внешнем ПО необходимо предусмотреть механизмы переподключения и переподписки (при необходимости)!  В OpenAPI V2 доступны \"Московская Биржа\" (MOEX) и \"Биржа СПБ\" (SPBX).   ### Доступные типы данных * Все сделки * Все заявки * Информация по инструментам * Котировки * Биржевые стаканы * Исторические данные * Позиции * Информация о клиенте  ### Поддерживаемые виды заявок  * рыночные  * лимитные  * стоп-лосс  * тейк-профит  * стоп-лосс лимит  * тейк-профит лимит  ### Ограничения по частоте запросов На текущий момент ограничений по количеству запросов в минуту нет, однако есть ограничение на общее количество подписок (сотни тысяч). При достижении лимита подписок клиент будет заблокирован и в течение нескольких минут не сможет создавать новые подписки. Уже существующие подписки продолжат работать. Сервер может обрабатывать \"тяжелые\" запросы (пример - история за все время) и запросы без авторизации с меньшим приоритетом.  ### Получение списка портфелей Получить список доступных портфелей можно из JWT токена Для получения списка доступных портфелей необходимо декодировать JWT токен. Портфели находятся в поле **portfolios**.  ## Авторизация  ### OAuth  **Внимание!** JWT и refresh token — равносильны логину и паролю. Их нужно скрывать от публичного доступа.  ### Для разработчиков сторонних приложений, в которых торговлю будут вести их пользователи.  Мы предоставляем сервис для авторизации сторониих приложений по стандарту OAuth 2.0. С примером приложения, использующего OAuth сервис для авторизации пользователей можно ознакомиться в разделе [Примеры](https://alor.dev/examples).  Список разрешений (scopes), которые могут быть выданы приложению: <table>   <tr>     <td><b>OrdersRead</b></td>     <td>Чтение выставленных заявок</td>   </tr>   <tr>     <td><b>OrdersCreate</b></td>     <td>Выставление заявок</td>   </tr>   <tr>     <td><b>Trades</b></td>     <td>Чтение совершенных сделок</td>   </tr>   <tr>     <td><b>Personal</b></td>     <td>Персональная информация: ФИО, почта и т.п.</td>   </tr>   <tr>     <td><b>Stats</b></td>     <td>Статистика: прибыль, средние цены и т.п.</td>   </tr> </table>  ### Для ведения операций от своего имени  Выписать себе **refresh token** для ведения операций от своего имени [можно здесь](https://alor.dev/open-api-tokens).  ## Краткое описание работы с авторизацией  Чтобы выполнить авторизованный запрос, добавьте в запрос заголовок с именем \"Authorization\" и значением, состоящим из префикса `\"Bearer \"` и валидного JWT токена.  Срок жизни JWT короткий: это сделано для безопасности.  Для большинства вариантов использования API мы рекоммендуем использовать механизм  **refresh token**.  Механизм  **refresh token**  позволяет получать JWT с новым сроком жизни. Для этого отправьте POST запрос на адрес `https://oauthdev.alor.ru/refresh?token={refreshToken}` *(тестовый контур)* или `https://oauth.alor.ru/refresh?token={refreshToken}` *(боевой контур)*. Если у **refresh token**  не истек срок жизни и не он не был отозван, то в теле ответа в поле AccessToken вернётся свежий JWT токен.   Срок жизни  **refresh token**, получаемого обычным способом — 1 месяц.   Срок жизни  **refresh token**, получаемого самостоятельным выписыванием — год.   ---   Если мы для вас не завели портфели для ведения торговли в игровом контуре, оставьте заявку на [openapi@alor.ru](mailto:openapi@alor.ru) или свяжитесь с нами в [телеграме](https://t.me/AlorOpenAPI).   Тестовый контур: `https://apidev.alor.ru`   Боевой контур: `https://api.alor.ru`   --- 
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
pub struct Position {
  #[serde(rename = "avgPrice")]
  ///Средняя цена
  avg_price: Decimal,  // 16.6 
  #[serde(rename = "brokerSymbol")]
  ///Пара Биржа:Тикер
  broker_symbol: String,  // MOEX:LKOH 
  #[serde(rename = "currentVolume")]
  ///Объём, рассчитанный по текущей цене
  current_volume: Decimal,  // 38798790 
  #[serde(rename = "dailyUnrealisedPl")]
  ///Нереализованная прибыль за день
  daily_unrealised_pl: Decimal,  // 2.8 
  #[serde(rename = "exchange")]
  
  exchange: Exchange, 
  #[serde(rename = "isCurrency")]
  ///`True` для валютных остатков (денег), `false` - для торговых инструментов
  is_currency: bool,  // false 
  #[serde(rename = "lotSize")]
  ///Размер лота
  lot_size: Decimal,  // 10.0 
  #[serde(rename = "open")]
  ///Агрегированное количество на момент открытия (начала торгов) (штуки)
  open: Decimal,  // 20.0 
  #[serde(rename = "openQtyBatch")]
  ///Агрегированное количество на момент открытия (начала торгов) (лоты)
  open_qty_batch: Decimal,  // 20.0 
  #[serde(rename = "openUnits")]
  ///Количество открытых позиций на момент открытия (начала торгов)
  open_units: Decimal,  // 30.0 
  #[serde(rename = "portfolio")]
  ///Идентификатор клиентского портфеля
  portfolio: String,  // D39004 
  #[serde(rename = "qty")]
  ///Агрегированное количество (лоты)
  qty: Decimal,  // 20.0 
  #[serde(rename = "qtyBatch")]
  ///Агрегированное количество TFuture
  qty_batch: Decimal,  // 20.0 
  #[serde(rename = "qtyT0")]
  ///Агрегированное количество T0 (штуки)
  qty_t0: Decimal,  // 20.0 
  #[serde(rename = "qtyT0Batch")]
  ///Агрегированное количество T0 (лоты)
  qty_t0_batch: Decimal,  // 20.0 
  #[serde(rename = "qtyT1")]
  ///Агрегированное количество T1 (штуки)
  qty_t1: Decimal,  // 20.0 
  #[serde(rename = "qtyT1Batch")]
  ///Агрегированное количество T1 (лоты)
  qty_t1_batch: Decimal,  // 20.0 
  #[serde(rename = "qtyT2")]
  ///Агрегированное количество T2 (штуки)
  qty_t2: Decimal,  // 20.0 
  #[serde(rename = "qtyT2Batch")]
  ///Агрегированное количество T2 (лоты)
  qty_t2_batch: Decimal,  // 20.0 
  #[serde(rename = "qtyTFuture")]
  ///Количество (штуки)
  qty_t_future: Decimal,  // 20.0 
  #[serde(rename = "qtyTFutureBatch")]
  ///Агрегированное количество TFuture (лоты)
  qty_t_future_batch: Decimal,  // 20.0 
  #[serde(rename = "qtyUnits")]
  ///Количество (штуки)
  qty_units: Decimal,  // 20.0 
  #[serde(rename = "shortName")]
  ///Короткое наименование
  short_name: String,  // ЛУКОЙЛ 
  #[serde(rename = "symbol")]
  ///Тикер (Код финансового инструмента)
  symbol: String,  // SBER 
  #[serde(rename = "unrealisedPl")]
  
  unrealised_pl: Decimal,  // 3250.0 
  #[serde(rename = "volume")]
  ///Объём, рассчитанный по средней цене
  volume: Decimal  // 38767080 
}

impl Position {
  pub fn new(avg_price: Decimal, broker_symbol: String, current_volume: Decimal, daily_unrealised_pl: Decimal, exchange: Exchange, is_currency: bool, lot_size: Decimal, open: Decimal, open_qty_batch: Decimal, open_units: Decimal, portfolio: String, qty: Decimal, qty_batch: Decimal, qty_t0: Decimal, qty_t0_batch: Decimal, qty_t1: Decimal, qty_t1_batch: Decimal, qty_t2: Decimal, qty_t2_batch: Decimal, qty_t_future: Decimal, qty_t_future_batch: Decimal, qty_units: Decimal, short_name: String, symbol: String, unrealised_pl: Decimal, volume: Decimal, ) -> Position {
    Position {
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
      portfolio: portfolio,
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

  pub fn with_avg_price(mut self, avg_price: Decimal) -> Position {
    self.avg_price = avg_price;
    self
  }
  ///Средняя цена
  pub fn avg_price(&self) -> &Decimal {
    &self.avg_price
  }


  pub fn set_broker_symbol(&mut self, broker_symbol: String) {
    self.broker_symbol = broker_symbol;
  }

  pub fn with_broker_symbol(mut self, broker_symbol: String) -> Position {
    self.broker_symbol = broker_symbol;
    self
  }
  ///Пара Биржа:Тикер
  pub fn broker_symbol(&self) -> &String {
    &self.broker_symbol
  }


  pub fn set_current_volume(&mut self, current_volume: Decimal) {
    self.current_volume = current_volume;
  }

  pub fn with_current_volume(mut self, current_volume: Decimal) -> Position {
    self.current_volume = current_volume;
    self
  }
  ///Объём, рассчитанный по текущей цене
  pub fn current_volume(&self) -> &Decimal {
    &self.current_volume
  }


  pub fn set_daily_unrealised_pl(&mut self, daily_unrealised_pl: Decimal) {
    self.daily_unrealised_pl = daily_unrealised_pl;
  }

  pub fn with_daily_unrealised_pl(mut self, daily_unrealised_pl: Decimal) -> Position {
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

  pub fn with_exchange(mut self, exchange: Exchange) -> Position {
    self.exchange = exchange;
    self
  }
  
  pub fn exchange(&self) -> &Exchange {
    &self.exchange
  }


  pub fn set_is_currency(&mut self, is_currency: bool) {
    self.is_currency = is_currency;
  }

  pub fn with_is_currency(mut self, is_currency: bool) -> Position {
    self.is_currency = is_currency;
    self
  }
  ///`True` для валютных остатков (денег), `false` - для торговых инструментов
  pub fn is_currency(&self) -> &bool {
    &self.is_currency
  }


  pub fn set_lot_size(&mut self, lot_size: Decimal) {
    self.lot_size = lot_size;
  }

  pub fn with_lot_size(mut self, lot_size: Decimal) -> Position {
    self.lot_size = lot_size;
    self
  }
  ///Размер лота
  pub fn lot_size(&self) -> &Decimal {
    &self.lot_size
  }


  pub fn set_open(&mut self, open: Decimal) {
    self.open = open;
  }

  pub fn with_open(mut self, open: Decimal) -> Position {
    self.open = open;
    self
  }
  ///Агрегированное количество на момент открытия (начала торгов) (штуки)
  pub fn open(&self) -> &Decimal {
    &self.open
  }


  pub fn set_open_qty_batch(&mut self, open_qty_batch: Decimal) {
    self.open_qty_batch = open_qty_batch;
  }

  pub fn with_open_qty_batch(mut self, open_qty_batch: Decimal) -> Position {
    self.open_qty_batch = open_qty_batch;
    self
  }
  ///Агрегированное количество на момент открытия (начала торгов) (лоты)
  pub fn open_qty_batch(&self) -> &Decimal {
    &self.open_qty_batch
  }


  pub fn set_open_units(&mut self, open_units: Decimal) {
    self.open_units = open_units;
  }

  pub fn with_open_units(mut self, open_units: Decimal) -> Position {
    self.open_units = open_units;
    self
  }
  ///Количество открытых позиций на момент открытия (начала торгов)
  pub fn open_units(&self) -> &Decimal {
    &self.open_units
  }


  pub fn set_portfolio(&mut self, portfolio: String) {
    self.portfolio = portfolio;
  }

  pub fn with_portfolio(mut self, portfolio: String) -> Position {
    self.portfolio = portfolio;
    self
  }
  ///Идентификатор клиентского портфеля
  pub fn portfolio(&self) -> &String {
    &self.portfolio
  }


  pub fn set_qty(&mut self, qty: Decimal) {
    self.qty = qty;
  }

  pub fn with_qty(mut self, qty: Decimal) -> Position {
    self.qty = qty;
    self
  }
  ///Агрегированное количество (лоты)
  pub fn qty(&self) -> &Decimal {
    &self.qty
  }


  pub fn set_qty_batch(&mut self, qty_batch: Decimal) {
    self.qty_batch = qty_batch;
  }

  pub fn with_qty_batch(mut self, qty_batch: Decimal) -> Position {
    self.qty_batch = qty_batch;
    self
  }
  ///Агрегированное количество TFuture
  pub fn qty_batch(&self) -> &Decimal {
    &self.qty_batch
  }


  pub fn set_qty_t0(&mut self, qty_t0: Decimal) {
    self.qty_t0 = qty_t0;
  }

  pub fn with_qty_t0(mut self, qty_t0: Decimal) -> Position {
    self.qty_t0 = qty_t0;
    self
  }
  ///Агрегированное количество T0 (штуки)
  pub fn qty_t0(&self) -> &Decimal {
    &self.qty_t0
  }


  pub fn set_qty_t0_batch(&mut self, qty_t0_batch: Decimal) {
    self.qty_t0_batch = qty_t0_batch;
  }

  pub fn with_qty_t0_batch(mut self, qty_t0_batch: Decimal) -> Position {
    self.qty_t0_batch = qty_t0_batch;
    self
  }
  ///Агрегированное количество T0 (лоты)
  pub fn qty_t0_batch(&self) -> &Decimal {
    &self.qty_t0_batch
  }


  pub fn set_qty_t1(&mut self, qty_t1: Decimal) {
    self.qty_t1 = qty_t1;
  }

  pub fn with_qty_t1(mut self, qty_t1: Decimal) -> Position {
    self.qty_t1 = qty_t1;
    self
  }
  ///Агрегированное количество T1 (штуки)
  pub fn qty_t1(&self) -> &Decimal {
    &self.qty_t1
  }


  pub fn set_qty_t1_batch(&mut self, qty_t1_batch: Decimal) {
    self.qty_t1_batch = qty_t1_batch;
  }

  pub fn with_qty_t1_batch(mut self, qty_t1_batch: Decimal) -> Position {
    self.qty_t1_batch = qty_t1_batch;
    self
  }
  ///Агрегированное количество T1 (лоты)
  pub fn qty_t1_batch(&self) -> &Decimal {
    &self.qty_t1_batch
  }


  pub fn set_qty_t2(&mut self, qty_t2: Decimal) {
    self.qty_t2 = qty_t2;
  }

  pub fn with_qty_t2(mut self, qty_t2: Decimal) -> Position {
    self.qty_t2 = qty_t2;
    self
  }
  ///Агрегированное количество T2 (штуки)
  pub fn qty_t2(&self) -> &Decimal {
    &self.qty_t2
  }


  pub fn set_qty_t2_batch(&mut self, qty_t2_batch: Decimal) {
    self.qty_t2_batch = qty_t2_batch;
  }

  pub fn with_qty_t2_batch(mut self, qty_t2_batch: Decimal) -> Position {
    self.qty_t2_batch = qty_t2_batch;
    self
  }
  ///Агрегированное количество T2 (лоты)
  pub fn qty_t2_batch(&self) -> &Decimal {
    &self.qty_t2_batch
  }


  pub fn set_qty_t_future(&mut self, qty_t_future: Decimal) {
    self.qty_t_future = qty_t_future;
  }

  pub fn with_qty_t_future(mut self, qty_t_future: Decimal) -> Position {
    self.qty_t_future = qty_t_future;
    self
  }
  ///Количество (штуки)
  pub fn qty_t_future(&self) -> &Decimal {
    &self.qty_t_future
  }


  pub fn set_qty_t_future_batch(&mut self, qty_t_future_batch: Decimal) {
    self.qty_t_future_batch = qty_t_future_batch;
  }

  pub fn with_qty_t_future_batch(mut self, qty_t_future_batch: Decimal) -> Position {
    self.qty_t_future_batch = qty_t_future_batch;
    self
  }
  ///Агрегированное количество TFuture (лоты)
  pub fn qty_t_future_batch(&self) -> &Decimal {
    &self.qty_t_future_batch
  }


  pub fn set_qty_units(&mut self, qty_units: Decimal) {
    self.qty_units = qty_units;
  }

  pub fn with_qty_units(mut self, qty_units: Decimal) -> Position {
    self.qty_units = qty_units;
    self
  }
  ///Количество (штуки)
  pub fn qty_units(&self) -> &Decimal {
    &self.qty_units
  }


  pub fn set_short_name(&mut self, short_name: String) {
    self.short_name = short_name;
  }

  pub fn with_short_name(mut self, short_name: String) -> Position {
    self.short_name = short_name;
    self
  }
  ///Короткое наименование
  pub fn short_name(&self) -> &String {
    &self.short_name
  }


  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> Position {
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

  pub fn with_unrealised_pl(mut self, unrealised_pl: Decimal) -> Position {
    self.unrealised_pl = unrealised_pl;
    self
  }
  
  pub fn unrealised_pl(&self) -> &Decimal {
    &self.unrealised_pl
  }


  pub fn set_volume(&mut self, volume: Decimal) {
    self.volume = volume;
  }

  pub fn with_volume(mut self, volume: Decimal) -> Position {
    self.volume = volume;
    self
  }
  ///Объём, рассчитанный по средней цене
  pub fn volume(&self) -> &Decimal {
    &self.volume
  }



  pub fn validate(&self) {
  }

}


