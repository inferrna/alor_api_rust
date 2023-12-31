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
pub struct HistoryObject {
  #[serde(rename = "close")]
  ///Цена при закрытии
  close: Decimal,  // 210.83 
  #[serde(rename = "high")]
  ///Максимальная цена
  high: Decimal,  // 176.02 
  #[serde(rename = "low")]
  ///Минимальная цена
  low: Decimal,  // 170.33 
  #[serde(rename = "open")]
  ///Цена при открытии
  open: Decimal,  // 210.82 
  #[serde(rename = "time")]
  ///Время (UTC) (Unix time seconds)
  time: i64,  // 1620220020 
  #[serde(rename = "volume")]
  ///Объём
  volume: i32  // 1944 
}

impl HistoryObject {
  pub fn new(close: Decimal, high: Decimal, low: Decimal, open: Decimal, time: i64, volume: i32, ) -> HistoryObject {
    HistoryObject {
      close: close,
      high: high,
      low: low,
      open: open,
      time: time,
      volume: volume
    }
  }

  pub fn set_close(&mut self, close: Decimal) {
    self.close = close;
  }

  pub fn with_close(mut self, close: Decimal) -> HistoryObject {
    self.close = close;
    self
  }
  ///Цена при закрытии
  pub fn close(&self) -> &Decimal {
    &self.close
  }


  pub fn set_high(&mut self, high: Decimal) {
    self.high = high;
  }

  pub fn with_high(mut self, high: Decimal) -> HistoryObject {
    self.high = high;
    self
  }
  ///Максимальная цена
  pub fn high(&self) -> &Decimal {
    &self.high
  }


  pub fn set_low(&mut self, low: Decimal) {
    self.low = low;
  }

  pub fn with_low(mut self, low: Decimal) -> HistoryObject {
    self.low = low;
    self
  }
  ///Минимальная цена
  pub fn low(&self) -> &Decimal {
    &self.low
  }


  pub fn set_open(&mut self, open: Decimal) {
    self.open = open;
  }

  pub fn with_open(mut self, open: Decimal) -> HistoryObject {
    self.open = open;
    self
  }
  ///Цена при открытии
  pub fn open(&self) -> &Decimal {
    &self.open
  }


  pub fn set_time(&mut self, time: i64) {
    self.time = time;
  }

  pub fn with_time(mut self, time: i64) -> HistoryObject {
    self.time = time;
    self
  }
  ///Время (UTC) (Unix time seconds)
  pub fn time(&self) -> &i64 {
    &self.time
  }


  pub fn set_volume(&mut self, volume: i32) {
    self.volume = volume;
  }

  pub fn with_volume(mut self, volume: i32) -> HistoryObject {
    self.volume = volume;
    self
  }
  ///Объём
  pub fn volume(&self) -> &i32 {
    &self.volume
  }



  pub fn validate(&self) {
  }

}


