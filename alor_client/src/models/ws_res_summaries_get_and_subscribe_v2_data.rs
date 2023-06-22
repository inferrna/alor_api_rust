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
/// WsResSummariesGetAndSubscribeV2Data : Сообщение с данными о позициях по деньгам

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
pub struct WsResSummariesGetAndSubscribeV2Data {
  #[serde(rename = "buyingPower")]
  ///Покупательская способность
  buying_power: Decimal,  // 5418.27 
  #[serde(rename = "buyingPowerAtMorning")]
  ///Покупательская способность (на утро)
  buying_power_at_morning: Decimal,  // 5410.89 
  #[serde(rename = "commission")]
  #[serde(default)]
  ///Суммарная комиссия (null для Срочного рынка)
  commission: Option<Decimal>,  // 24.21 
  #[serde(rename = "initialMargin")]
  ///Начальная маржа
  initial_margin: Decimal,  // 3539.0 
  #[serde(rename = "portfolioEvaluation")]
  ///Ликвидный портфель
  portfolio_evaluation: Decimal,  // 8857.0 
  #[serde(rename = "portfolioLiquidationValue")]
  ///Оценка портфеля
  portfolio_liquidation_value: Decimal,  // 10714.0 
  #[serde(rename = "profit")]
  ///Прибыль
  profit: Decimal,  // 93.0 
  #[serde(rename = "profitRate")]
  ///Относительная прибыль
  profit_rate: Decimal,  // 0.87 
  #[serde(rename = "riskBeforeForcePositionClosing")]
  ///Риск до закрытия
  risk_before_force_position_closing: Decimal  // 7088.0 
}

impl WsResSummariesGetAndSubscribeV2Data {
  pub fn new(buying_power: Decimal, buying_power_at_morning: Decimal, initial_margin: Decimal, portfolio_evaluation: Decimal, portfolio_liquidation_value: Decimal, profit: Decimal, profit_rate: Decimal, risk_before_force_position_closing: Decimal, ) -> WsResSummariesGetAndSubscribeV2Data {
    WsResSummariesGetAndSubscribeV2Data {
      buying_power: buying_power,
      buying_power_at_morning: buying_power_at_morning,
      commission: None,
      initial_margin: initial_margin,
      portfolio_evaluation: portfolio_evaluation,
      portfolio_liquidation_value: portfolio_liquidation_value,
      profit: profit,
      profit_rate: profit_rate,
      risk_before_force_position_closing: risk_before_force_position_closing
    }
  }

  pub fn set_buying_power(&mut self, buying_power: Decimal) {
    self.buying_power = buying_power;
  }

  pub fn with_buying_power(mut self, buying_power: Decimal) -> WsResSummariesGetAndSubscribeV2Data {
    self.buying_power = buying_power;
    self
  }
  ///Покупательская способность
  pub fn buying_power(&self) -> &Decimal {
    &self.buying_power
  }


  pub fn set_buying_power_at_morning(&mut self, buying_power_at_morning: Decimal) {
    self.buying_power_at_morning = buying_power_at_morning;
  }

  pub fn with_buying_power_at_morning(mut self, buying_power_at_morning: Decimal) -> WsResSummariesGetAndSubscribeV2Data {
    self.buying_power_at_morning = buying_power_at_morning;
    self
  }
  ///Покупательская способность (на утро)
  pub fn buying_power_at_morning(&self) -> &Decimal {
    &self.buying_power_at_morning
  }


  pub fn set_commission(&mut self, commission: Decimal) {
    self.commission = Some(commission);
  }

  pub fn with_commission(mut self, commission: Decimal) -> WsResSummariesGetAndSubscribeV2Data {
    self.commission = Some(commission);
    self
  }
  ///Суммарная комиссия (null для Срочного рынка)
  pub fn commission(&self) -> Option<&Decimal> {
    self.commission.as_ref()
  }

  pub fn reset_commission(&mut self) {
    self.commission = None;
  }

  pub fn set_initial_margin(&mut self, initial_margin: Decimal) {
    self.initial_margin = initial_margin;
  }

  pub fn with_initial_margin(mut self, initial_margin: Decimal) -> WsResSummariesGetAndSubscribeV2Data {
    self.initial_margin = initial_margin;
    self
  }
  ///Начальная маржа
  pub fn initial_margin(&self) -> &Decimal {
    &self.initial_margin
  }


  pub fn set_portfolio_evaluation(&mut self, portfolio_evaluation: Decimal) {
    self.portfolio_evaluation = portfolio_evaluation;
  }

  pub fn with_portfolio_evaluation(mut self, portfolio_evaluation: Decimal) -> WsResSummariesGetAndSubscribeV2Data {
    self.portfolio_evaluation = portfolio_evaluation;
    self
  }
  ///Ликвидный портфель
  pub fn portfolio_evaluation(&self) -> &Decimal {
    &self.portfolio_evaluation
  }


  pub fn set_portfolio_liquidation_value(&mut self, portfolio_liquidation_value: Decimal) {
    self.portfolio_liquidation_value = portfolio_liquidation_value;
  }

  pub fn with_portfolio_liquidation_value(mut self, portfolio_liquidation_value: Decimal) -> WsResSummariesGetAndSubscribeV2Data {
    self.portfolio_liquidation_value = portfolio_liquidation_value;
    self
  }
  ///Оценка портфеля
  pub fn portfolio_liquidation_value(&self) -> &Decimal {
    &self.portfolio_liquidation_value
  }


  pub fn set_profit(&mut self, profit: Decimal) {
    self.profit = profit;
  }

  pub fn with_profit(mut self, profit: Decimal) -> WsResSummariesGetAndSubscribeV2Data {
    self.profit = profit;
    self
  }
  ///Прибыль
  pub fn profit(&self) -> &Decimal {
    &self.profit
  }


  pub fn set_profit_rate(&mut self, profit_rate: Decimal) {
    self.profit_rate = profit_rate;
  }

  pub fn with_profit_rate(mut self, profit_rate: Decimal) -> WsResSummariesGetAndSubscribeV2Data {
    self.profit_rate = profit_rate;
    self
  }
  ///Относительная прибыль
  pub fn profit_rate(&self) -> &Decimal {
    &self.profit_rate
  }


  pub fn set_risk_before_force_position_closing(&mut self, risk_before_force_position_closing: Decimal) {
    self.risk_before_force_position_closing = risk_before_force_position_closing;
  }

  pub fn with_risk_before_force_position_closing(mut self, risk_before_force_position_closing: Decimal) -> WsResSummariesGetAndSubscribeV2Data {
    self.risk_before_force_position_closing = risk_before_force_position_closing;
    self
  }
  ///Риск до закрытия
  pub fn risk_before_force_position_closing(&self) -> &Decimal {
    &self.risk_before_force_position_closing
  }



  pub fn validate(&self) {
  }

}


