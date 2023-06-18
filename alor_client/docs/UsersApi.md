# {{classname}}

All URIs are relative to *https://apidev.alor.ru*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dev_get_all_orders**](UsersApi.md#dev_get_all_orders) | **GET** md/v2/clients/{exchange}/{portfolio}/orders | Получение информации о всех заявках
[**dev_get_all_positions**](UsersApi.md#dev_get_all_positions) | **GET** md/v2/Clients/{exchange}/{portfolio}/positions | Получение информации о позициях
[**dev_get_all_stop_orders**](UsersApi.md#dev_get_all_stop_orders) | **GET** md/v2/clients/{exchange}/{portfolio}/stoporders | Получение информации о стоп-заявках
[**dev_get_all_trades**](UsersApi.md#dev_get_all_trades) | **GET** md/v2/Clients/{exchange}/{portfolio}/trades | Получение информации о сделках
[**dev_get_one_order**](UsersApi.md#dev_get_one_order) | **GET** md/v2/clients/{exchange}/{portfolio}/orders/{orderId} | Получение информации о выбранной заявке
[**dev_get_one_position**](UsersApi.md#dev_get_one_position) | **GET** md/v2/Clients/{exchange}/{portfolio}/positions/{symbol} | Получение информации о позициях выбранного инструмента
[**dev_get_one_stop_order**](UsersApi.md#dev_get_one_stop_order) | **GET** md/v2/clients/{exchange}/{portfolio}/stoporders/{orderId} | Получение информации о выбранной стоп-заявке
[**dev_get_ticker_trades**](UsersApi.md#dev_get_ticker_trades) | **GET** md/v2/Clients/{exchange}/{portfolio}/{ticker}/trades | Получение информации о сделках по выбранному инструменту
[**dev_user_portfolio**](UsersApi.md#dev_user_portfolio) | **GET** client/v1.0/users/{username}/portfolios | Получение списка серверов портфелей
[**exchange_portfolio_money**](UsersApi.md#exchange_portfolio_money) | **GET** md/v2/clients/legacy/{exchange}/{portfolio}/money | Получение информации по деньгам для выбранного портфеля
[**exchange_portfolio_summary**](UsersApi.md#exchange_portfolio_summary) | **GET** md/v2/clients/{exchange}/{portfolio}/summary | Получение информации о портфеле
[**fortsrisk**](UsersApi.md#fortsrisk) | **GET** md/v2/Clients/{exchange}/{portfolio}/fortsrisk | Получение информации о рисках на срочном рынке
[**risk**](UsersApi.md#risk) | **GET** md/v2/Clients/{exchange}/{portfolio}/risk | Получение информации о рисках
[**trade_stats**](UsersApi.md#trade_stats) | **GET** md/stats/{exchange}/{portfolio}/history/trades | Получение истории сделок
[**trade_stats_by_symbol**](UsersApi.md#trade_stats_by_symbol) | **GET** md/stats/{exchange}/{portfolio}/history/trades/{symbol} | Получение истории сделок (один тикер)

# **dev_get_all_orders**
> Orders dev_get_all_orders(ctx, exchange, portfolio, optional)
Получение информации о всех заявках

Запрос информации о всех заявках

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**Orders**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_get_all_positions**
> Positions dev_get_all_positions(ctx, exchange, portfolio, optional)
Получение информации о позициях

Запрос информации о позициях

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 
 **without_currency** | [**SchemaEnum**](.md)| Исключить из ответа все денежные инструменты, по умолчанию false | 

### Return type

[**Positions**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_get_all_stop_orders**
> StopordersWarp dev_get_all_stop_orders(ctx, exchange, portfolio, optional)
Получение информации о стоп-заявках

Запрос информации о всех стоп-заявках

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**StopordersWarp**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_get_all_trades**
> Trades dev_get_all_trades(ctx, exchange, portfolio, optional)
Получение информации о сделках

Запрос информации о сделках (только за текущую торговую сессию)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**Trades**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_get_one_order**
> Order dev_get_one_order(ctx, exchange, portfolio, order_id, optional)
Получение информации о выбранной заявке

Запрос информации о выбранной заявке

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
  **order_id** | **i32**| Идентификатор заявки | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **order_id** | **i32**| Идентификатор заявки | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**Order**](order.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_get_one_position**
> Position dev_get_one_position(ctx, exchange, portfolio, symbol, optional)
Получение информации о позициях выбранного инструмента

Запрос информации о позициях выбранного инструмента

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
  **symbol** | **String**| Тикер (Код финансового инструмента) | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **symbol** | **String**| Тикер (Код финансового инструмента) | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**Position**](position.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_get_one_stop_order**
> StoporderWarp dev_get_one_stop_order(ctx, exchange, portfolio, order_id, optional)
Получение информации о выбранной стоп-заявке

Запрос информации о выбранной стоп-заявке

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
  **order_id** | **i32**| Идентификатор стоп-заявки | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **order_id** | **i32**| Идентификатор стоп-заявки | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**StoporderWarp**](stoporderWarp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_get_ticker_trades**
> Trades dev_get_ticker_trades(ctx, exchange, portfolio, ticker, optional)
Получение информации о сделках по выбранному инструменту

Запрос информации о сделках по выбранному инструменту

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
  **ticker** | **String**| Тикер (Код финансового инструмента) | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **ticker** | **String**| Тикер (Код финансового инструмента) | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**Trades**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_user_portfolio**
> ServersInfo dev_user_portfolio(ctx, username)
Получение списка серверов портфелей

Получение списка серверов. В ответе в поле tradeServerCode содержится значение которое надо использовать. Не являются частью API торговой системы.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **username** | **String**| Имя пользователя | 

### Return type

[**ServersInfo**](servers_info.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **exchange_portfolio_money**
> Money exchange_portfolio_money(ctx, exchange, portfolio, optional)
Получение информации по деньгам для выбранного портфеля

Запрос информации о позиции по деньгам. Вызов существует для обратной совместимости с API v1, предпочтительно использовать другие вызовы (/summary, /risk, /positions)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**Money**](money.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **exchange_portfolio_summary**
> Summary exchange_portfolio_summary(ctx, exchange, portfolio, optional)
Получение информации о портфеле

Запрос сводной информации о портфеле

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**Summary**](summary.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fortsrisk**
> Fortsrisk fortsrisk(ctx, exchange, portfolio, optional)
Получение информации о рисках на срочном рынке

Запрос информации о рисках на срочном рынке для выбранного портфеля

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**Fortsrisk**](fortsrisk.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **risk**
> Risk risk(ctx, exchange, portfolio, optional)
Получение информации о рисках

Запрос информации о рисках

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**Risk**](risk.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **trade_stats**
> Trades trade_stats(ctx, exchange, portfolio, optional)
Получение истории сделок

Запрос списка сделок за предыдущие дни (не более 1000 сделок за один запрос)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **date_from** | **NaiveDate**| Начиная с какой даты отдавать историю сделок | 
 **from** | **i32**| Начиная с какого ID (номера сделки) отдавать историю сделок | 
 **limit** | **i32**| Количество возвращаемых записей (максимум 1000) | 
 **descending** | **bool**| Флаг обратной сортировки выдачи | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**Trades**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **trade_stats_by_symbol**
> Trades trade_stats_by_symbol(ctx, exchange, portfolio, symbol, optional)
Получение истории сделок (один тикер)

Запрос списка сделок за предыдущие дни (не более 1000 сделок за один запрос) по одному инструменту. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
  **symbol** | **String**| Фильтр по инструменту | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **symbol** | **String**| Фильтр по инструменту | 
 **date_from** | **NaiveDate**| Начиная с какой даты отдавать историю сделок | 
 **from** | **i32**| Начиная с какого ID (номера сделки) отдавать историю сделок | 
 **limit** | **i32**| Количество возвращаемых записей (максимум 1000) | 
 **descending** | **bool**| Флаг обратной сортировки выдачи | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**Trades**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

