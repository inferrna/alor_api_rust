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
pub struct Fortsrisk {
  #[serde(rename = "balanceMoney")]
  ///Сальдо денежных торговых переводов за текущую сессию (поле будет удалено в будущих обновлениях)
  balance_money: Decimal,  // 1.93 
  #[serde(rename = "fee")]
  ///Списанный сбор
  fee: Decimal,  // 651717.0 
  #[serde(rename = "isLimitsSet")]
  ///Наличие установленных денежного и залогового лимитов
  is_limits_set: bool,  // false 
  #[serde(rename = "moneyAmount")]
  ///Общее количество рублей и дисконтированных в рубли залогов
  money_amount: Decimal,  // 199313.0 
  #[serde(rename = "moneyBlocked")]
  ///Средства, заблокированные под ГО
  money_blocked: Decimal,  // 12560.0 
  #[serde(rename = "moneyFree")]
  ///Свободные средства. Сумма рублей и залогов, дисконтированных в рубли, доступная для открытия позиций. (`MoneyFree` = `MoneyAmount` + `VmInterCl` – `MoneyBlocked` – `VmReserve` – `Fee`)
  money_free: Decimal,  // 452404.0 
  #[serde(rename = "moneyOld")]
  ///Общее количество рублей и дисконтированных в рубли залогов на начало сессии
  money_old: Decimal,  // 651717.0 
  #[serde(rename = "moneyPledgeAmount")]
  ///Сумма залогов, дисконтированных в рубли
  money_pledge_amount: Decimal,  // 552061.0 
  #[serde(rename = "portfolio")]
  ///Идентификатор клиентского портфеля
  portfolio: String,  // D39004 
  #[serde(rename = "varMargin")]
  ///`VmCurrentPositions` + `VmInterCl`
  var_margin: Decimal,  // 552061.0 
  #[serde(rename = "vmCurrentPositions")]
  ///Сагрегированная вармаржа по текущим позициям
  vm_current_positions: Decimal,  // 199313.0 
  #[serde(rename = "vmInterCl")]
  ///Вариационная маржа, списанная или полученная в пром. клиринг
  vm_inter_cl: Decimal  // 651717.0 
}

impl Fortsrisk {
  pub fn new(balance_money: Decimal, fee: Decimal, is_limits_set: bool, money_amount: Decimal, money_blocked: Decimal, money_free: Decimal, money_old: Decimal, money_pledge_amount: Decimal, portfolio: String, var_margin: Decimal, vm_current_positions: Decimal, vm_inter_cl: Decimal, ) -> Fortsrisk {
    Fortsrisk {
      balance_money: balance_money,
      fee: fee,
      is_limits_set: is_limits_set,
      money_amount: money_amount,
      money_blocked: money_blocked,
      money_free: money_free,
      money_old: money_old,
      money_pledge_amount: money_pledge_amount,
      portfolio: portfolio,
      var_margin: var_margin,
      vm_current_positions: vm_current_positions,
      vm_inter_cl: vm_inter_cl
    }
  }

  pub fn set_balance_money(&mut self, balance_money: Decimal) {
    self.balance_money = balance_money;
  }

  pub fn with_balance_money(mut self, balance_money: Decimal) -> Fortsrisk {
    self.balance_money = balance_money;
    self
  }
  ///Сальдо денежных торговых переводов за текущую сессию (поле будет удалено в будущих обновлениях)
  pub fn balance_money(&self) -> &Decimal {
    &self.balance_money
  }


  pub fn set_fee(&mut self, fee: Decimal) {
    self.fee = fee;
  }

  pub fn with_fee(mut self, fee: Decimal) -> Fortsrisk {
    self.fee = fee;
    self
  }
  ///Списанный сбор
  pub fn fee(&self) -> &Decimal {
    &self.fee
  }


  pub fn set_is_limits_set(&mut self, is_limits_set: bool) {
    self.is_limits_set = is_limits_set;
  }

  pub fn with_is_limits_set(mut self, is_limits_set: bool) -> Fortsrisk {
    self.is_limits_set = is_limits_set;
    self
  }
  ///Наличие установленных денежного и залогового лимитов
  pub fn is_limits_set(&self) -> &bool {
    &self.is_limits_set
  }


  pub fn set_money_amount(&mut self, money_amount: Decimal) {
    self.money_amount = money_amount;
  }

  pub fn with_money_amount(mut self, money_amount: Decimal) -> Fortsrisk {
    self.money_amount = money_amount;
    self
  }
  ///Общее количество рублей и дисконтированных в рубли залогов
  pub fn money_amount(&self) -> &Decimal {
    &self.money_amount
  }


  pub fn set_money_blocked(&mut self, money_blocked: Decimal) {
    self.money_blocked = money_blocked;
  }

  pub fn with_money_blocked(mut self, money_blocked: Decimal) -> Fortsrisk {
    self.money_blocked = money_blocked;
    self
  }
  ///Средства, заблокированные под ГО
  pub fn money_blocked(&self) -> &Decimal {
    &self.money_blocked
  }


  pub fn set_money_free(&mut self, money_free: Decimal) {
    self.money_free = money_free;
  }

  pub fn with_money_free(mut self, money_free: Decimal) -> Fortsrisk {
    self.money_free = money_free;
    self
  }
  ///Свободные средства. Сумма рублей и залогов, дисконтированных в рубли, доступная для открытия позиций. (`MoneyFree` = `MoneyAmount` + `VmInterCl` – `MoneyBlocked` – `VmReserve` – `Fee`)
  pub fn money_free(&self) -> &Decimal {
    &self.money_free
  }


  pub fn set_money_old(&mut self, money_old: Decimal) {
    self.money_old = money_old;
  }

  pub fn with_money_old(mut self, money_old: Decimal) -> Fortsrisk {
    self.money_old = money_old;
    self
  }
  ///Общее количество рублей и дисконтированных в рубли залогов на начало сессии
  pub fn money_old(&self) -> &Decimal {
    &self.money_old
  }


  pub fn set_money_pledge_amount(&mut self, money_pledge_amount: Decimal) {
    self.money_pledge_amount = money_pledge_amount;
  }

  pub fn with_money_pledge_amount(mut self, money_pledge_amount: Decimal) -> Fortsrisk {
    self.money_pledge_amount = money_pledge_amount;
    self
  }
  ///Сумма залогов, дисконтированных в рубли
  pub fn money_pledge_amount(&self) -> &Decimal {
    &self.money_pledge_amount
  }


  pub fn set_portfolio(&mut self, portfolio: String) {
    self.portfolio = portfolio;
  }

  pub fn with_portfolio(mut self, portfolio: String) -> Fortsrisk {
    self.portfolio = portfolio;
    self
  }
  ///Идентификатор клиентского портфеля
  pub fn portfolio(&self) -> &String {
    &self.portfolio
  }


  pub fn set_var_margin(&mut self, var_margin: Decimal) {
    self.var_margin = var_margin;
  }

  pub fn with_var_margin(mut self, var_margin: Decimal) -> Fortsrisk {
    self.var_margin = var_margin;
    self
  }
  ///`VmCurrentPositions` + `VmInterCl`
  pub fn var_margin(&self) -> &Decimal {
    &self.var_margin
  }


  pub fn set_vm_current_positions(&mut self, vm_current_positions: Decimal) {
    self.vm_current_positions = vm_current_positions;
  }

  pub fn with_vm_current_positions(mut self, vm_current_positions: Decimal) -> Fortsrisk {
    self.vm_current_positions = vm_current_positions;
    self
  }
  ///Сагрегированная вармаржа по текущим позициям
  pub fn vm_current_positions(&self) -> &Decimal {
    &self.vm_current_positions
  }


  pub fn set_vm_inter_cl(&mut self, vm_inter_cl: Decimal) {
    self.vm_inter_cl = vm_inter_cl;
  }

  pub fn with_vm_inter_cl(mut self, vm_inter_cl: Decimal) -> Fortsrisk {
    self.vm_inter_cl = vm_inter_cl;
    self
  }
  ///Вариационная маржа, списанная или полученная в пром. клиринг
  pub fn vm_inter_cl(&self) -> &Decimal {
    &self.vm_inter_cl
  }



  pub fn validate(&self) {
  }

}


