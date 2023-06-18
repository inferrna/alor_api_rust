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
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

use hyper;
use serde_json;
use serde_json::Value;
use tokio::runtime::Runtime;
use futures;
use futures::{Future, Stream};
use hyper::Body;
use hyper::body::Bytes;
use hyper::body::HttpBody;
use std::str::FromStr;
use chrono::{NaiveDate, NaiveDateTime, DateTime, FixedOffset, Utc, SecondsFormat};
use crate::{OutlinePrint};
use crate::models::*;
use super::{Error, configuration};
use headers::{Authorization, Header};
use headers::authorization::Credentials;
use rust_decimal::Decimal;

pub struct OtherApiClient<C: hyper::client::connect::Connect + Clone + Send + Sync> {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static> OtherApiClient<C> {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> OtherApiClient<C> {
        OtherApiClient {
            configuration: configuration,
        }
    }
}

#[async_trait::async_trait]
pub trait OtherApi {
    async fn local_time(&self, ) -> Result<Time, Error<serde_json::Value>>;
}

#[async_trait::async_trait]
impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static>OtherApi for OtherApiClient<C> {
 ///
 /// Запрос текущего UTC времени в формате Unix
 ///
 /// Запрос текущего UTC времени в формате Unix Time Seconds. Если этот запрос выполнен без авторизации, то будет возвращено время, которое было 15 минут назад.
 ///
 /// # Arguments
 ///
    async fn local_time(&self, ) -> Result<Time, Error<serde_json::Value>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref token) = configuration.oauth_access_token {
            auth_headers.insert("Authorization".to_owned(), format!("Bearer {}", token));
        }
        let method = hyper::Method::GET;

        let query_string = {
            let mut api_query = ::url::form_urlencoded::Serializer::new(String::new());
            let has_query_params = false;
            for (key, val) in &auth_query {
                api_query.append_pair(key, val);
            }
            if has_query_params || auth_query.len()>0  {
                format!("/?{}", api_query.finish())
            } else {
                "".to_string()
            }
        };
        let uri_str = format!("{}/md/v2/time{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        //dbg!(&uri_str);

        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req =
            hyper::Request::builder()
                .method(method)
                .uri(uri);

        let headers = req.headers_mut().unwrap();

        if let Some(ref user_agent) = configuration.user_agent {
            headers.insert(hyper::header::USER_AGENT, user_agent.parse().unwrap());
        }


        for (key, val) in auth_headers {
            headers.insert(
                hyper::header::HeaderName::from_str(key.as_ref()).unwrap(),
                val.parse().unwrap(),
            );
        }

        let somebody = Body::empty();

        let req = req.body(somebody).unwrap();

        let res = configuration
            .client.request(req)
            .await
            .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

        let mut res = res?;

        let status = res.status();
        let mut res_body: Vec<u8> = vec![];

        while let Some(chunk) = res.body_mut().data().await {
            let mut chunk_vec = chunk.unwrap().to_vec();
            res_body.append(chunk_vec.as_mut());
        }

        //Uncomment to see what went wrong
/*
        let string_result = std::str::from_utf8(&res_body).unwrap();
        let value_result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&string_result);
        if let Ok(json_value) = value_result {
            //Valid json, invalid structure, pretty-printed output
            eprintln!("{}", serde_json::to_string_pretty(&json_value).unwrap());
        } else {
            //Invalid json, raw output
            dbg!(&string_result);
        }
*/
        let res_body =
            if status.is_success() {
                Ok(res_body)
            } else {
                Err(Error::from((status, res_body.borrow())))
            };

        let mut res_body = res_body?;

        let res_body =
            serde_json::from_slice(res_body.borrow())
            .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

        res_body
    }

}