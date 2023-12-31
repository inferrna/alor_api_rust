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
pub struct Summary {
  #[serde(rename = "buyingPower")]
  ///Покупательская способность
  buying_power: Decimal,  // 452404 
  #[serde(rename = "buyingPowerAtMorning")]
  ///Покупательская способность на утро
  buying_power_at_morning: Decimal,  // 439844.15 
  #[serde(rename = "commission")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(default)]
  ///Суммарная комиссия (null для Срочного рынка)
  commission: Option<Decimal>,  // 24.21 
  #[serde(rename = "initialMargin")]
  ///Маржа
  initial_margin: Decimal,  // 199313.0 
  #[serde(rename = "portfolioEvaluation")]
  ///Ликвидный портфель
  portfolio_evaluation: Decimal,  // 651717.0 
  #[serde(rename = "portfolioLiquidationValue")]
  ///Оценка портфеля
  portfolio_liquidation_value: Decimal,  // 651717.0 
  #[serde(rename = "profit")]
  ///Прибыль за сегодня
  profit: Decimal,  // 12560.0 
  #[serde(rename = "profitRate")]
  ///Норма прибыли, %
  profit_rate: Decimal,  // 1.93 
  #[serde(rename = "riskBeforeForcePositionClosing")]
  ///Риск до закрытия
  risk_before_force_position_closing: Decimal  // 552061.0 
}

impl Summary {
  pub fn new(buying_power: Decimal, buying_power_at_morning: Decimal, initial_margin: Decimal, portfolio_evaluation: Decimal, portfolio_liquidation_value: Decimal, profit: Decimal, profit_rate: Decimal, risk_before_force_position_closing: Decimal, ) -> Summary {
    Summary {
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

  pub fn with_buying_power(mut self, buying_power: Decimal) -> Summary {
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

  pub fn with_buying_power_at_morning(mut self, buying_power_at_morning: Decimal) -> Summary {
    self.buying_power_at_morning = buying_power_at_morning;
    self
  }
  ///Покупательская способность на утро
  pub fn buying_power_at_morning(&self) -> &Decimal {
    &self.buying_power_at_morning
  }


  pub fn set_commission(&mut self, commission: Decimal) {
    self.commission = Some(commission);
  }

  pub fn with_commission(mut self, commission: Decimal) -> Summary {
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

  pub fn with_initial_margin(mut self, initial_margin: Decimal) -> Summary {
    self.initial_margin = initial_margin;
    self
  }
  ///Маржа
  pub fn initial_margin(&self) -> &Decimal {
    &self.initial_margin
  }


  pub fn set_portfolio_evaluation(&mut self, portfolio_evaluation: Decimal) {
    self.portfolio_evaluation = portfolio_evaluation;
  }

  pub fn with_portfolio_evaluation(mut self, portfolio_evaluation: Decimal) -> Summary {
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

  pub fn with_portfolio_liquidation_value(mut self, portfolio_liquidation_value: Decimal) -> Summary {
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

  pub fn with_profit(mut self, profit: Decimal) -> Summary {
    self.profit = profit;
    self
  }
  ///Прибыль за сегодня
  pub fn profit(&self) -> &Decimal {
    &self.profit
  }


  pub fn set_profit_rate(&mut self, profit_rate: Decimal) {
    self.profit_rate = profit_rate;
  }

  pub fn with_profit_rate(mut self, profit_rate: Decimal) -> Summary {
    self.profit_rate = profit_rate;
    self
  }
  ///Норма прибыли, %
  pub fn profit_rate(&self) -> &Decimal {
    &self.profit_rate
  }


  pub fn set_risk_before_force_position_closing(&mut self, risk_before_force_position_closing: Decimal) {
    self.risk_before_force_position_closing = risk_before_force_position_closing;
  }

  pub fn with_risk_before_force_position_closing(mut self, risk_before_force_position_closing: Decimal) -> Summary {
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


