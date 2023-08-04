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
pub struct RiskRate {
  #[serde(rename = "assetType")]
  ///Тип актива
  asset_type: String,  // stock 
  #[serde(rename = "createdAt")]
  ///Время добавления ставки риска
  created_at: String,  // 2021-08-24T10:13:01.6584899 
  #[serde(rename = "currencyCode")]
  ///Код валюты расчетов
  currency_code: String,  // RUB 
  #[serde(rename = "exchange")]
  
  exchange: Exchange, 
  #[serde(rename = "id")]
  ///Id записи
  id: i64,  // 22229 
  #[serde(rename = "instrument")]
  ///Инструмент
  instrument: String,  // SBERP 
  #[serde(rename = "isDirect")]
  ///Является ли зависимость инструмента к базовому активу прямой или обратной.
  is_direct: bool,  // false 
  #[serde(rename = "isMarginal")]
  ///Доступен ли данный инструмент в маржу. Т.е. есть ли он в списке маржинальных инструментов брокера.
  is_marginal: bool,  // true 
  #[serde(rename = "isShortSellPossible")]
  ///Разрешен ли шорт по бумаге. True если да.
  is_short_sell_possible: bool,  // true 
  #[serde(rename = "isin")]
  ///ISIN инструмента. Если есть.
  isin: String,  // RU0009029557 
  #[serde(rename = "rateDown")]
  ///Ставка риска понижения цены. Применяется для лонгов.
  rate_down: Decimal,  // 10 
  #[serde(rename = "rateSymmetric")]
  ///Симметричная ставка риска. Приведена для справки, не используется
  rate_symmetric: Decimal,  // 10 
  #[serde(rename = "rateUp")]
  ///Ставка риска повышения цены. Применяется для шортов.
  rate_up: Decimal,  // 20 
  #[serde(rename = "riskCategoryId")]
  ///Категория риска. 
  risk_category_id: i32,  // 2 
  #[serde(rename = "securityRiskCategoryId")]
  #[serde(default)]
  ///Id категории бумаги для категоризации. 
  security_risk_category_id: Option<Decimal>,  // 1 
  #[serde(rename = "setName")]
  #[serde(default)]
  ///Чаще всего будет null. Поле показывает к множеству инструменту принадлежит данный инструмент.
  set_name: Option<String>,  // SBER 
  #[serde(rename = "setRate")]
  ///Ставка риска множества
  set_rate: Decimal,  // 0 
  #[serde(rename = "underlyingAsset")]
  #[serde(default)]
  ///Чаще всего будет null. Поле показывает к какому базовому инструменту принадлежит данный инструмент.
  underlying_asset: Option<String>,  // SBER 
  #[serde(rename = "updatedAt")]
  //Uncomment this also to deal with limited rfc support on server side
  //#[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
  ///Время последнего обновления ставки риска
  updated_at: DateTime<Utc> 
}

impl RiskRate {
  pub fn new(asset_type: String, created_at: String, currency_code: String, exchange: Exchange, id: i64, instrument: String, is_direct: bool, is_marginal: bool, is_short_sell_possible: bool, isin: String, rate_down: Decimal, rate_symmetric: Decimal, rate_up: Decimal, risk_category_id: i32, set_rate: Decimal, updated_at: DateTime<Utc>, ) -> RiskRate {
    RiskRate {
      asset_type: asset_type,
      created_at: created_at,
      currency_code: currency_code,
      exchange: exchange,
      id: id,
      instrument: instrument,
      is_direct: is_direct,
      is_marginal: is_marginal,
      is_short_sell_possible: is_short_sell_possible,
      isin: isin,
      rate_down: rate_down,
      rate_symmetric: rate_symmetric,
      rate_up: rate_up,
      risk_category_id: risk_category_id,
      security_risk_category_id: None,
      set_name: None,
      set_rate: set_rate,
      underlying_asset: None,
      updated_at: updated_at
    }
  }

  pub fn set_asset_type(&mut self, asset_type: String) {
    self.asset_type = asset_type;
  }

  pub fn with_asset_type(mut self, asset_type: String) -> RiskRate {
    self.asset_type = asset_type;
    self
  }
  ///Тип актива
  pub fn asset_type(&self) -> &String {
    &self.asset_type
  }


  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = created_at;
  }

  pub fn with_created_at(mut self, created_at: String) -> RiskRate {
    self.created_at = created_at;
    self
  }
  ///Время добавления ставки риска
  pub fn created_at(&self) -> &String {
    &self.created_at
  }


  pub fn set_currency_code(&mut self, currency_code: String) {
    self.currency_code = currency_code;
  }

  pub fn with_currency_code(mut self, currency_code: String) -> RiskRate {
    self.currency_code = currency_code;
    self
  }
  ///Код валюты расчетов
  pub fn currency_code(&self) -> &String {
    &self.currency_code
  }


  pub fn set_exchange(&mut self, exchange: Exchange) {
    self.exchange = exchange;
  }

  pub fn with_exchange(mut self, exchange: Exchange) -> RiskRate {
    self.exchange = exchange;
    self
  }
  
  pub fn exchange(&self) -> &Exchange {
    &self.exchange
  }


  pub fn set_id(&mut self, id: i64) {
    self.id = id;
  }

  pub fn with_id(mut self, id: i64) -> RiskRate {
    self.id = id;
    self
  }
  ///Id записи
  pub fn id(&self) -> &i64 {
    &self.id
  }


  pub fn set_instrument(&mut self, instrument: String) {
    self.instrument = instrument;
  }

  pub fn with_instrument(mut self, instrument: String) -> RiskRate {
    self.instrument = instrument;
    self
  }
  ///Инструмент
  pub fn instrument(&self) -> &String {
    &self.instrument
  }


  pub fn set_is_direct(&mut self, is_direct: bool) {
    self.is_direct = is_direct;
  }

  pub fn with_is_direct(mut self, is_direct: bool) -> RiskRate {
    self.is_direct = is_direct;
    self
  }
  ///Является ли зависимость инструмента к базовому активу прямой или обратной.
  pub fn is_direct(&self) -> &bool {
    &self.is_direct
  }


  pub fn set_is_marginal(&mut self, is_marginal: bool) {
    self.is_marginal = is_marginal;
  }

  pub fn with_is_marginal(mut self, is_marginal: bool) -> RiskRate {
    self.is_marginal = is_marginal;
    self
  }
  ///Доступен ли данный инструмент в маржу. Т.е. есть ли он в списке маржинальных инструментов брокера.
  pub fn is_marginal(&self) -> &bool {
    &self.is_marginal
  }


  pub fn set_is_short_sell_possible(&mut self, is_short_sell_possible: bool) {
    self.is_short_sell_possible = is_short_sell_possible;
  }

  pub fn with_is_short_sell_possible(mut self, is_short_sell_possible: bool) -> RiskRate {
    self.is_short_sell_possible = is_short_sell_possible;
    self
  }
  ///Разрешен ли шорт по бумаге. True если да.
  pub fn is_short_sell_possible(&self) -> &bool {
    &self.is_short_sell_possible
  }


  pub fn set_isin(&mut self, isin: String) {
    self.isin = isin;
  }

  pub fn with_isin(mut self, isin: String) -> RiskRate {
    self.isin = isin;
    self
  }
  ///ISIN инструмента. Если есть.
  pub fn isin(&self) -> &String {
    &self.isin
  }


  pub fn set_rate_down(&mut self, rate_down: Decimal) {
    self.rate_down = rate_down;
  }

  pub fn with_rate_down(mut self, rate_down: Decimal) -> RiskRate {
    self.rate_down = rate_down;
    self
  }
  ///Ставка риска понижения цены. Применяется для лонгов.
  pub fn rate_down(&self) -> &Decimal {
    &self.rate_down
  }


  pub fn set_rate_symmetric(&mut self, rate_symmetric: Decimal) {
    self.rate_symmetric = rate_symmetric;
  }

  pub fn with_rate_symmetric(mut self, rate_symmetric: Decimal) -> RiskRate {
    self.rate_symmetric = rate_symmetric;
    self
  }
  ///Симметричная ставка риска. Приведена для справки, не используется
  pub fn rate_symmetric(&self) -> &Decimal {
    &self.rate_symmetric
  }


  pub fn set_rate_up(&mut self, rate_up: Decimal) {
    self.rate_up = rate_up;
  }

  pub fn with_rate_up(mut self, rate_up: Decimal) -> RiskRate {
    self.rate_up = rate_up;
    self
  }
  ///Ставка риска повышения цены. Применяется для шортов.
  pub fn rate_up(&self) -> &Decimal {
    &self.rate_up
  }


  pub fn set_risk_category_id(&mut self, risk_category_id: i32) {
    self.risk_category_id = risk_category_id;
  }

  pub fn with_risk_category_id(mut self, risk_category_id: i32) -> RiskRate {
    self.risk_category_id = risk_category_id;
    self
  }
  ///Категория риска. 
  pub fn risk_category_id(&self) -> &i32 {
    &self.risk_category_id
  }


  pub fn set_security_risk_category_id(&mut self, security_risk_category_id: Decimal) {
    self.security_risk_category_id = Some(security_risk_category_id);
  }

  pub fn with_security_risk_category_id(mut self, security_risk_category_id: Decimal) -> RiskRate {
    self.security_risk_category_id = Some(security_risk_category_id);
    self
  }
  ///Id категории бумаги для категоризации. 
  pub fn security_risk_category_id(&self) -> Option<&Decimal> {
    self.security_risk_category_id.as_ref()
  }

  pub fn reset_security_risk_category_id(&mut self) {
    self.security_risk_category_id = None;
  }

  pub fn set_set_name(&mut self, set_name: String) {
    self.set_name = Some(set_name);
  }

  pub fn with_set_name(mut self, set_name: String) -> RiskRate {
    self.set_name = Some(set_name);
    self
  }
  ///Чаще всего будет null. Поле показывает к множеству инструменту принадлежит данный инструмент.
  pub fn set_name(&self) -> Option<&String> {
    self.set_name.as_ref()
  }

  pub fn reset_set_name(&mut self) {
    self.set_name = None;
  }

  pub fn set_set_rate(&mut self, set_rate: Decimal) {
    self.set_rate = set_rate;
  }

  pub fn with_set_rate(mut self, set_rate: Decimal) -> RiskRate {
    self.set_rate = set_rate;
    self
  }
  ///Ставка риска множества
  pub fn set_rate(&self) -> &Decimal {
    &self.set_rate
  }


  pub fn set_underlying_asset(&mut self, underlying_asset: String) {
    self.underlying_asset = Some(underlying_asset);
  }

  pub fn with_underlying_asset(mut self, underlying_asset: String) -> RiskRate {
    self.underlying_asset = Some(underlying_asset);
    self
  }
  ///Чаще всего будет null. Поле показывает к какому базовому инструменту принадлежит данный инструмент.
  pub fn underlying_asset(&self) -> Option<&String> {
    self.underlying_asset.as_ref()
  }

  pub fn reset_underlying_asset(&mut self) {
    self.underlying_asset = None;
  }

  pub fn set_updated_at(&mut self, updated_at: DateTime<Utc>) {
    self.updated_at = updated_at;
  }

  pub fn with_updated_at(mut self, updated_at: DateTime<Utc>) -> RiskRate {
    self.updated_at = updated_at;
    self
  }
  ///Время последнего обновления ставки риска
  pub fn updated_at(&self) -> &DateTime<Utc> {
    &self.updated_at
  }



  pub fn validate(&self) {
  }

}


