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
pub struct Security {
  #[serde(rename = "ISIN")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(default)]
  ///Идентификатор ценной бумаги согласно стандарту ISO 6166
  isin: Option<String>,  // RU000A1014L8 
  #[serde(rename = "board")]
  ///Код режима торгов (Борд)
  board: String,  // TQBR 
  #[serde(rename = "cancellation")]
  ///Дата и время (UTC) окончания действия
  cancellation: String,  // 2018-09-03T00:00:00 
  #[serde(rename = "cfiCode")]
  ///Тип ценной бумаги согласно стандарту ISO 10962
  cfi_code: String,  // ESXXXX 
  #[serde(rename = "complexProductCategory")]
  
  complex_product_category: ComplexProductCategory, 
  #[serde(rename = "currency")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(default)]
  ///Валюта
  currency: Option<String>,  // RUB 
  #[serde(rename = "description")]
  ///Краткое описание инструмента
  description: String,  // Сбербанк России ПАО ао 
  #[serde(rename = "exchange")]
  
  exchange: Exchange, 
  #[serde(rename = "facevalue")]
  ///Номинальная стоимость
  facevalue: Decimal,  // 100.0 
  #[serde(rename = "lotsize")]
  ///Размер лота
  lotsize: Decimal,  // 10.0 
  #[serde(rename = "marginbuy")]
  ///Цена маржинальной покупки (заемные средства)
  marginbuy: Decimal,  // 6707.86 
  #[serde(rename = "marginrate")]
  ///Отношение цены маржинальной покупки к цене последней сделки
  marginrate: Decimal,  // 89.3428 
  #[serde(rename = "marginsell")]
  ///Цена маржинальной продажи (заемные средства)
  marginsell: Decimal,  // 6707.86 
  #[serde(rename = "minstep")]
  ///Минимальный шаг цены
  minstep: Decimal,  // 0.01 
  #[serde(rename = "priceMax")]
  ///Максимальная цена
  price_max: Decimal,  // 176.02 
  #[serde(rename = "priceMin")]
  ///Минимальная цена
  price_min: Decimal,  // 170.33 
  #[serde(rename = "pricestep")]
  ///Минимальный шаг цены, выраженный в рублях
  pricestep: Decimal,  // 6.30202 
  #[serde(rename = "primary_board")]
  ///Код режима торгов
  primary_board: String,  // TQBR 
  #[serde(rename = "rating")]
  
  rating: Decimal,  // 195613886 
  #[serde(rename = "shortname")]
  ///Краткое наименование инструмента
  shortname: String,  // Сбербанк 
  #[serde(rename = "symbol")]
  ///Тикер (Код финансового инструмента)
  symbol: String,  // SBER 
  #[serde(rename = "theorPrice")]
  ///Теоретическая цена опциона
  theor_price: Decimal,  // 0.0 
  #[serde(rename = "theorPriceLimit")]
  ///Теоретическая цена опциона с учетом лимитов
  theor_price_limit: Decimal,  // 0.0 
  #[serde(rename = "tradingStatus")]
  ///Торговый статус инструмента:   * `18` - Нет торгов / торги закрыты   * `118` - Период открытия   * `103` - Период закрытия   * `2` - Перерыв в торгах   * `17` - Нормальный период торгов   * `102` - Аукцион закрытия   * `106` - Аукцион крупных пакетов   * `107` - Дискретный аукцион   * `119` - Аукцион открытия   * `120` - Период торгов по цене аукциона закрытия 
  trading_status: i32,  // 17 
  #[serde(rename = "tradingStatusInfo")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(default)]
  ///Описание торгового статуса инструмента
  trading_status_info: Option<String>,  // нормальный период торгов 
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(default)]
  ///Тип
  rtype: Option<String>,  // CS 
  #[serde(rename = "volatility")]
  ///Волатильность
  volatility: Decimal,  // 0.0 
  #[serde(rename = "yield")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(default)]
  ///Доходность, рассчитанная по цене сделки
  ryield: Option<i32> 
}

impl Security {
  pub fn new(board: String, cancellation: String, cfi_code: String, complex_product_category: ComplexProductCategory, description: String, exchange: Exchange, facevalue: Decimal, lotsize: Decimal, marginbuy: Decimal, marginrate: Decimal, marginsell: Decimal, minstep: Decimal, price_max: Decimal, price_min: Decimal, pricestep: Decimal, primary_board: String, rating: Decimal, shortname: String, symbol: String, theor_price: Decimal, theor_price_limit: Decimal, trading_status: i32, volatility: Decimal, ) -> Security {
    Security {
      isin: None,
      board: board,
      cancellation: cancellation,
      cfi_code: cfi_code,
      complex_product_category: complex_product_category,
      currency: None,
      description: description,
      exchange: exchange,
      facevalue: facevalue,
      lotsize: lotsize,
      marginbuy: marginbuy,
      marginrate: marginrate,
      marginsell: marginsell,
      minstep: minstep,
      price_max: price_max,
      price_min: price_min,
      pricestep: pricestep,
      primary_board: primary_board,
      rating: rating,
      shortname: shortname,
      symbol: symbol,
      theor_price: theor_price,
      theor_price_limit: theor_price_limit,
      trading_status: trading_status,
      trading_status_info: None,
      rtype: None,
      volatility: volatility,
      ryield: None
    }
  }

  pub fn set_isin(&mut self, isin: String) {
    self.isin = Some(isin);
  }

  pub fn with_isin(mut self, isin: String) -> Security {
    self.isin = Some(isin);
    self
  }
  ///Идентификатор ценной бумаги согласно стандарту ISO 6166
  pub fn isin(&self) -> Option<&String> {
    self.isin.as_ref()
  }

  pub fn reset_isin(&mut self) {
    self.isin = None;
  }

  pub fn set_board(&mut self, board: String) {
    self.board = board;
  }

  pub fn with_board(mut self, board: String) -> Security {
    self.board = board;
    self
  }
  ///Код режима торгов (Борд)
  pub fn board(&self) -> &String {
    &self.board
  }


  pub fn set_cancellation(&mut self, cancellation: String) {
    self.cancellation = cancellation;
  }

  pub fn with_cancellation(mut self, cancellation: String) -> Security {
    self.cancellation = cancellation;
    self
  }
  ///Дата и время (UTC) окончания действия
  pub fn cancellation(&self) -> &String {
    &self.cancellation
  }


  pub fn set_cfi_code(&mut self, cfi_code: String) {
    self.cfi_code = cfi_code;
  }

  pub fn with_cfi_code(mut self, cfi_code: String) -> Security {
    self.cfi_code = cfi_code;
    self
  }
  ///Тип ценной бумаги согласно стандарту ISO 10962
  pub fn cfi_code(&self) -> &String {
    &self.cfi_code
  }


  pub fn set_complex_product_category(&mut self, complex_product_category: ComplexProductCategory) {
    self.complex_product_category = complex_product_category;
  }

  pub fn with_complex_product_category(mut self, complex_product_category: ComplexProductCategory) -> Security {
    self.complex_product_category = complex_product_category;
    self
  }
  
  pub fn complex_product_category(&self) -> &ComplexProductCategory {
    &self.complex_product_category
  }


  pub fn set_currency(&mut self, currency: String) {
    self.currency = Some(currency);
  }

  pub fn with_currency(mut self, currency: String) -> Security {
    self.currency = Some(currency);
    self
  }
  ///Валюта
  pub fn currency(&self) -> Option<&String> {
    self.currency.as_ref()
  }

  pub fn reset_currency(&mut self) {
    self.currency = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = description;
  }

  pub fn with_description(mut self, description: String) -> Security {
    self.description = description;
    self
  }
  ///Краткое описание инструмента
  pub fn description(&self) -> &String {
    &self.description
  }


  pub fn set_exchange(&mut self, exchange: Exchange) {
    self.exchange = exchange;
  }

  pub fn with_exchange(mut self, exchange: Exchange) -> Security {
    self.exchange = exchange;
    self
  }
  
  pub fn exchange(&self) -> &Exchange {
    &self.exchange
  }


  pub fn set_facevalue(&mut self, facevalue: Decimal) {
    self.facevalue = facevalue;
  }

  pub fn with_facevalue(mut self, facevalue: Decimal) -> Security {
    self.facevalue = facevalue;
    self
  }
  ///Номинальная стоимость
  pub fn facevalue(&self) -> &Decimal {
    &self.facevalue
  }


  pub fn set_lotsize(&mut self, lotsize: Decimal) {
    self.lotsize = lotsize;
  }

  pub fn with_lotsize(mut self, lotsize: Decimal) -> Security {
    self.lotsize = lotsize;
    self
  }
  ///Размер лота
  pub fn lotsize(&self) -> &Decimal {
    &self.lotsize
  }


  pub fn set_marginbuy(&mut self, marginbuy: Decimal) {
    self.marginbuy = marginbuy;
  }

  pub fn with_marginbuy(mut self, marginbuy: Decimal) -> Security {
    self.marginbuy = marginbuy;
    self
  }
  ///Цена маржинальной покупки (заемные средства)
  pub fn marginbuy(&self) -> &Decimal {
    &self.marginbuy
  }


  pub fn set_marginrate(&mut self, marginrate: Decimal) {
    self.marginrate = marginrate;
  }

  pub fn with_marginrate(mut self, marginrate: Decimal) -> Security {
    self.marginrate = marginrate;
    self
  }
  ///Отношение цены маржинальной покупки к цене последней сделки
  pub fn marginrate(&self) -> &Decimal {
    &self.marginrate
  }


  pub fn set_marginsell(&mut self, marginsell: Decimal) {
    self.marginsell = marginsell;
  }

  pub fn with_marginsell(mut self, marginsell: Decimal) -> Security {
    self.marginsell = marginsell;
    self
  }
  ///Цена маржинальной продажи (заемные средства)
  pub fn marginsell(&self) -> &Decimal {
    &self.marginsell
  }


  pub fn set_minstep(&mut self, minstep: Decimal) {
    self.minstep = minstep;
  }

  pub fn with_minstep(mut self, minstep: Decimal) -> Security {
    self.minstep = minstep;
    self
  }
  ///Минимальный шаг цены
  pub fn minstep(&self) -> &Decimal {
    &self.minstep
  }


  pub fn set_price_max(&mut self, price_max: Decimal) {
    self.price_max = price_max;
  }

  pub fn with_price_max(mut self, price_max: Decimal) -> Security {
    self.price_max = price_max;
    self
  }
  ///Максимальная цена
  pub fn price_max(&self) -> &Decimal {
    &self.price_max
  }


  pub fn set_price_min(&mut self, price_min: Decimal) {
    self.price_min = price_min;
  }

  pub fn with_price_min(mut self, price_min: Decimal) -> Security {
    self.price_min = price_min;
    self
  }
  ///Минимальная цена
  pub fn price_min(&self) -> &Decimal {
    &self.price_min
  }


  pub fn set_pricestep(&mut self, pricestep: Decimal) {
    self.pricestep = pricestep;
  }

  pub fn with_pricestep(mut self, pricestep: Decimal) -> Security {
    self.pricestep = pricestep;
    self
  }
  ///Минимальный шаг цены, выраженный в рублях
  pub fn pricestep(&self) -> &Decimal {
    &self.pricestep
  }


  pub fn set_primary_board(&mut self, primary_board: String) {
    self.primary_board = primary_board;
  }

  pub fn with_primary_board(mut self, primary_board: String) -> Security {
    self.primary_board = primary_board;
    self
  }
  ///Код режима торгов
  pub fn primary_board(&self) -> &String {
    &self.primary_board
  }


  pub fn set_rating(&mut self, rating: Decimal) {
    self.rating = rating;
  }

  pub fn with_rating(mut self, rating: Decimal) -> Security {
    self.rating = rating;
    self
  }
  
  pub fn rating(&self) -> &Decimal {
    &self.rating
  }


  pub fn set_shortname(&mut self, shortname: String) {
    self.shortname = shortname;
  }

  pub fn with_shortname(mut self, shortname: String) -> Security {
    self.shortname = shortname;
    self
  }
  ///Краткое наименование инструмента
  pub fn shortname(&self) -> &String {
    &self.shortname
  }


  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> Security {
    self.symbol = symbol;
    self
  }
  ///Тикер (Код финансового инструмента)
  pub fn symbol(&self) -> &String {
    &self.symbol
  }


  pub fn set_theor_price(&mut self, theor_price: Decimal) {
    self.theor_price = theor_price;
  }

  pub fn with_theor_price(mut self, theor_price: Decimal) -> Security {
    self.theor_price = theor_price;
    self
  }
  ///Теоретическая цена опциона
  pub fn theor_price(&self) -> &Decimal {
    &self.theor_price
  }


  pub fn set_theor_price_limit(&mut self, theor_price_limit: Decimal) {
    self.theor_price_limit = theor_price_limit;
  }

  pub fn with_theor_price_limit(mut self, theor_price_limit: Decimal) -> Security {
    self.theor_price_limit = theor_price_limit;
    self
  }
  ///Теоретическая цена опциона с учетом лимитов
  pub fn theor_price_limit(&self) -> &Decimal {
    &self.theor_price_limit
  }


  pub fn set_trading_status(&mut self, trading_status: i32) {
    self.trading_status = trading_status;
  }

  pub fn with_trading_status(mut self, trading_status: i32) -> Security {
    self.trading_status = trading_status;
    self
  }
  ///Торговый статус инструмента:   * `18` - Нет торгов / торги закрыты   * `118` - Период открытия   * `103` - Период закрытия   * `2` - Перерыв в торгах   * `17` - Нормальный период торгов   * `102` - Аукцион закрытия   * `106` - Аукцион крупных пакетов   * `107` - Дискретный аукцион   * `119` - Аукцион открытия   * `120` - Период торгов по цене аукциона закрытия 
  pub fn trading_status(&self) -> &i32 {
    &self.trading_status
  }


  pub fn set_trading_status_info(&mut self, trading_status_info: String) {
    self.trading_status_info = Some(trading_status_info);
  }

  pub fn with_trading_status_info(mut self, trading_status_info: String) -> Security {
    self.trading_status_info = Some(trading_status_info);
    self
  }
  ///Описание торгового статуса инструмента
  pub fn trading_status_info(&self) -> Option<&String> {
    self.trading_status_info.as_ref()
  }

  pub fn reset_trading_status_info(&mut self) {
    self.trading_status_info = None;
  }

  pub fn set_rtype(&mut self, rtype: String) {
    self.rtype = Some(rtype);
  }

  pub fn with_rtype(mut self, rtype: String) -> Security {
    self.rtype = Some(rtype);
    self
  }
  ///Тип
  pub fn rtype(&self) -> Option<&String> {
    self.rtype.as_ref()
  }

  pub fn reset_rtype(&mut self) {
    self.rtype = None;
  }

  pub fn set_volatility(&mut self, volatility: Decimal) {
    self.volatility = volatility;
  }

  pub fn with_volatility(mut self, volatility: Decimal) -> Security {
    self.volatility = volatility;
    self
  }
  ///Волатильность
  pub fn volatility(&self) -> &Decimal {
    &self.volatility
  }


  pub fn set_ryield(&mut self, ryield: i32) {
    self.ryield = Some(ryield);
  }

  pub fn with_ryield(mut self, ryield: i32) -> Security {
    self.ryield = Some(ryield);
    self
  }
  ///Доходность, рассчитанная по цене сделки
  pub fn ryield(&self) -> Option<&i32> {
    self.ryield.as_ref()
  }

  pub fn reset_ryield(&mut self) {
    self.ryield = None;
  }


  pub fn validate(&self) {
  }

}


