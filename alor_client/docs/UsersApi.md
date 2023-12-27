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
[**exchange_portfolio_summary**](UsersApi.md#exchange_portfolio_summary) | **GET** md/v2/clients/{exchange}/{portfolio}/summary | Получение информации о портфеле
[**fortsrisk**](UsersApi.md#fortsrisk) | **GET** md/v2/Clients/{exchange}/{portfolio}/fortsrisk | Получение информации о рисках на срочном рынке
[**risk**](UsersApi.md#risk) | **GET** md/v2/Clients/{exchange}/{portfolio}/risk | Получение информации о рисках
[**trade_stats**](UsersApi.md#trade_stats) | **GET** md/stats/{exchange}/{portfolio}/history/trades | Получение истории сделок
[**trade_stats_by_symbol**](UsersApi.md#trade_stats_by_symbol) | **GET** md/stats/{exchange}/{portfolio}/history/trades/{symbol} | Получение истории сделок (один тикер)

# **dev_get_all_orders**
> Vec<Order> dev_get_all_orders(ctx, exchange, portfolio, optional)
Получение информации о всех заявках

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Возвращает информацию о всех заявках для указанного `portfolio`, созданных на заданной в параметре `exchange` бирже. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **portfolio** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **portfolio** | **String**|  | 
 **format** | [**Format**](.md)|  | 

### Return type

[**Vec<Order>**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_get_all_positions**
> Vec<Position> dev_get_all_positions(ctx, exchange, portfolio, optional)
Получение информации о позициях

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Возвращает информацию обо всех позициях указанного `portfolio`. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **portfolio** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **portfolio** | **String**|  | 
 **format** | [**Format**](.md)|  | 
 **without_currency** | **bool**|  | 

### Return type

[**Vec<Position>**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_get_all_stop_orders**
> Vec<StoporderWarp> dev_get_all_stop_orders(ctx, exchange, portfolio, optional)
Получение информации о стоп-заявках

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос информации о всех стоп-заявках 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **portfolio** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **portfolio** | **String**|  | 
 **format** | [**Format**](.md)|  | 

### Return type

[**Vec<StoporderWarp>**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_get_all_trades**
> Vec<Trade> dev_get_all_trades(ctx, exchange, portfolio, optional)
Получение информации о сделках

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос информации о сделках (только за текущую торговую сессию) 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **portfolio** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **portfolio** | **String**|  | 
 **format** | [**Format**](.md)|  | 

### Return type

[**Vec<Trade>**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_get_one_order**
> Order dev_get_one_order(ctx, exchange, portfolio, order_id, optional)
Получение информации о выбранной заявке

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Возвращает информацию о выбранной в параметре `orderId` заявке. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **portfolio** | **String**|  | 
  **order_id** | **i64**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **portfolio** | **String**|  | 
 **order_id** | **i64**|  | 
 **format** | [**Format**](.md)|  | 

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

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос информации о позициях выбранного инструмента 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **portfolio** | **String**|  | 
  **symbol** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **portfolio** | **String**|  | 
 **symbol** | **String**|  | 
 **format** | [**Format**](.md)|  | 

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

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос информации о выбранной стоп-заявке 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **portfolio** | **String**|  | 
  **order_id** | **i64**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **portfolio** | **String**|  | 
 **order_id** | **i64**|  | 
 **format** | [**Format**](.md)|  | 

### Return type

[**StoporderWarp**](stoporderWarp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_get_ticker_trades**
> Vec<Trade> dev_get_ticker_trades(ctx, exchange, portfolio, ticker, optional)
Получение информации о сделках по выбранному инструменту

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос информации о сделках по выбранному инструменту 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **portfolio** | **String**|  | 
  **ticker** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **portfolio** | **String**|  | 
 **ticker** | **String**|  | 
 **format** | [**Format**](.md)|  | 

### Return type

[**Vec<Trade>**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **exchange_portfolio_summary**
> Summary exchange_portfolio_summary(ctx, exchange, portfolio, optional)
Получение информации о портфеле

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос сводной информации о портфеле 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **portfolio** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **portfolio** | **String**|  | 
 **format** | [**Format**](.md)|  | 

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

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос информации о рисках на срочном рынке для выбранного портфеля 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **portfolio** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **portfolio** | **String**|  | 
 **format** | [**Format**](.md)|  | 

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

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос информации о рисках 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **portfolio** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **portfolio** | **String**|  | 
 **format** | [**Format**](.md)|  | 

### Return type

[**Risk**](risk.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **trade_stats**
> Vec<Trade> trade_stats(ctx, exchange, portfolio, optional)
Получение истории сделок

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос списка сделок за предыдущие дни (не более 1000 сделок за один запрос) 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **portfolio** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **portfolio** | **String**|  | 
 **date_from** | **NaiveDate**|  | 
 **from** | **i64**|  | 
 **limit** | **i32**|  | 
 **descending** | **bool**|  | 
 **format** | [**Format**](.md)|  | 

### Return type

[**Vec<Trade>**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **trade_stats_by_symbol**
> Vec<Trade> trade_stats_by_symbol(ctx, exchange, portfolio, symbol, optional)
Получение истории сделок (один тикер)

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос списка сделок за предыдущие дни (не более 1000 сделок за один запрос) по одному инструменту.  

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **portfolio** | **String**|  | 
  **symbol** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **portfolio** | **String**|  | 
 **symbol** | **String**|  | 
 **date_from** | **NaiveDate**|  | 
 **from** | **i64**|  | 
 **limit** | **i32**|  | 
 **descending** | **bool**|  | 
 **format** | [**Format**](.md)|  | 

### Return type

[**Vec<Trade>**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

