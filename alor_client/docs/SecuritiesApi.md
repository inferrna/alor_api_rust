# {{classname}}

All URIs are relative to *https://apidev.alor.ru*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dev_history**](SecuritiesApi.md#dev_history) | **GET** md/v2/history | Запрос истории для выбранных биржи и инструмента
[**dev_orderbook_exchang_seccode**](SecuritiesApi.md#dev_orderbook_exchang_seccode) | **GET** md/v2/orderbooks/{exchange}/{seccode} | Получение информации о биржевом стакане
[**dev_quotes**](SecuritiesApi.md#dev_quotes) | **GET** md/v2/Securities/{symbols}/quotes | Получение информации о котировках для выбранных инструментов
[**dev_securities_futures**](SecuritiesApi.md#dev_securities_futures) | **GET** md/v2/Securities/{exchange}/{symbol}/actualFuturesQuote | Получение котировки по ближайшему фьючерсу (код)
[**dev_securities_search**](SecuritiesApi.md#dev_securities_search) | **GET** md/v2/Securities | Получение информации о торговых инструментах
[**dev_securities_search_all_trades**](SecuritiesApi.md#dev_securities_search_all_trades) | **GET** md/v2/Securities/{exchange}/{symbol}/alltrades | Получение информации о всех сделках по ценным бумагам за сегодня
[**dev_securities_search_all_trades_history**](SecuritiesApi.md#dev_securities_search_all_trades_history) | **GET** md/v2/Securities/{exchange}/{symbol}/alltrades/history | Получение исторической информации о всех сделках по ценным бумагам
[**dev_securities_search_exchange**](SecuritiesApi.md#dev_securities_search_exchange) | **GET** md/v2/Securities/{exchange} | Получение информации о торговых инструментах на выбранной бирже
[**dev_securities_search_exchange_code**](SecuritiesApi.md#dev_securities_search_exchange_code) | **GET** md/v2/Securities/{exchange}/{symbol} | Получение информации о выбранном финансовом инструменте
[**risk_rates**](SecuritiesApi.md#risk_rates) | **GET** md/v2/risk/rates | Запрос ставок риска

# **dev_history**
> History dev_history(ctx, symbol, exchange, tf, from, to, optional)
Запрос истории для выбранных биржи и инструмента

**Запрос может быть выполнен без авторизации**. При отправке анонимного запроса вернутся данные, бывшие актуальными 15 минут назад.  Запрос истории рынка для выбранных биржи и финансового инструмента. Данные имеют задержку в 15 минут, если запрос не авторизован. Для авторизованных клиентов задержка не применяется. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **symbol** | **String**|  | 
  **exchange** | [**Exchange**](.md)|  | 
  **tf** | [**Duration**](.md)|  | 
  **from** | **i64**|  | 
  **to** | **i64**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **String**|  | 
 **exchange** | [**Exchange**](.md)|  | 
 **tf** | [**Duration**](.md)|  | 
 **from** | **i64**|  | 
 **to** | **i64**|  | 
 **untraded** | **bool**|  | 
 **format** | [**Format**](.md)|  | 

### Return type

[**History**](history.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_orderbook_exchang_seccode**
> Orderbook dev_orderbook_exchang_seccode(ctx, exchange, seccode, optional)
Получение информации о биржевом стакане

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос биржевого стакана 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **seccode** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **seccode** | **String**|  | 
 **depth** | **i32**|  | [default to 20]
 **format** | [**Format**](.md)|  | 

### Return type

[**Orderbook**](orderbook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_quotes**
> Vec<Symbol> dev_quotes(ctx, symbols, optional)
Получение информации о котировках для выбранных инструментов

**Запрос может быть выполнен без авторизации**. При отправке анонимного запроса вернутся данные, бывшие актуальными 15 минут назад.  Запрос информации о котировках для выбранных инструментов и бирж 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **symbols** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbols** | **String**|  | 
 **format** | [**Format**](.md)|  | 

### Return type

[**Vec<Symbol>**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_securities_futures**
> Vec<Symbol> dev_securities_futures(ctx, exchange, symbol, optional)
Получение котировки по ближайшему фьючерсу (код)

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос котировки по ближайшему фьючерсу (только по коду, без даты) 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **symbol** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **symbol** | **String**|  | 
 **format** | [**Format**](.md)|  | 

### Return type

[**Vec<Symbol>**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_securities_search**
> Vec<Security> dev_securities_search(ctx, query, optional)
Получение информации о торговых инструментах

**Запрос может быть выполнен без авторизации**. При отправке анонимного запроса вернутся данные, бывшие актуальными 15 минут назад.  Запрос информации о торговых инструментах 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **query** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **query** | **String**|  | 
 **limit** | **i32**|  | 
 **offset** | **i32**|  | 
 **sector** | [**Market**](.md)|  | 
 **cficode** | **String**|  | 
 **exchange** | [**Exchange**](.md)|  | 
 **format** | [**Format**](.md)|  | 

### Return type

[**Vec<Security>**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_securities_search_all_trades**
> Vec<Alltrade> dev_securities_search_all_trades(ctx, exchange, symbol, optional)
Получение информации о всех сделках по ценным бумагам за сегодня

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запросить данные о всех сделках (лента) по ценным бумагам за сегодняшний день 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **symbol** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **symbol** | **String**|  | 
 **format** | [**Format**](.md)|  | 
 **from** | **i64**|  | 
 **to** | **i64**|  | 
 **from_id** | **i64**|  | 
 **to_id** | **i64**|  | 
 **take** | **i32**|  | 
 **descending** | **bool**|  | 
 **include_virtual_trades** | **bool**|  | 

### Return type

[**Vec<Alltrade>**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_securities_search_all_trades_history**
> Alltradeshistory dev_securities_search_all_trades_history(ctx, exchange, symbol, limit, optional)
Получение исторической информации о всех сделках по ценным бумагам

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запросить данные о сделках (лента) по ценным бумагам за исторический период (за текущий день сделки не отдаются) 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **symbol** | **String**|  | 
  **limit** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **symbol** | **String**|  | 
 **limit** | **i32**|  | 
 **from** | **i64**|  | 
 **to** | **i64**|  | 
 **offset** | **i32**|  | 
 **format** | [**Format**](.md)|  | 

### Return type

[**Alltradeshistory**](alltradeshistory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_securities_search_exchange**
> Vec<Security> dev_securities_search_exchange(ctx, exchange, optional)
Получение информации о торговых инструментах на выбранной бирже

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос информации об инструментах на выбранной бирже 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **format** | [**Format**](.md)|  | 
 **market** | [**Market**](.md)|  | 
 **include_old** | **bool**|  | 
 **limit** | **i32**|  | 
 **offset** | **i32**|  | 

### Return type

[**Vec<Security>**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_securities_search_exchange_code**
> Security dev_securities_search_exchange_code(ctx, exchange, symbol, optional)
Получение информации о выбранном финансовом инструменте

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос информации о выбранном финансовом инструменте на бирже 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
  **symbol** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **symbol** | **String**|  | 
 **format** | [**Format**](.md)|  | 

### Return type

[**Security**](security.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **risk_rates**
> RiskRates risk_rates(ctx, exchange, optional)
Запрос ставок риска

**Запрос может быть выполнен без авторизации**. При отправке анонимного запроса вернутся данные, бывшие актуальными 15 минут назад.  Получение ставок риска для маржинальной торговли. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)|  | 
 **ticker** | **String**|  | 
 **risk_category_id** | **i32**|  | 
 **search** | **String**|  | 

### Return type

[**RiskRates**](riskRates.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

