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
[**dev_securities_search_exchange**](SecuritiesApi.md#dev_securities_search_exchange) | **GET** md/v2/Securities/{exchange} | Получение информации о торговых инструментах на выбранной бирже
[**dev_securities_search_exchange_code**](SecuritiesApi.md#dev_securities_search_exchange_code) | **GET** md/v2/Securities/{exchange}/{symbol} | Получение информации о выбранном финансовом инструменте
[**risk_rates**](SecuritiesApi.md#risk_rates) | **GET** md/v2/risk/rates | Запрос ставок риска

# **dev_history**
> History dev_history(ctx, symbol, exchange, tf, from, to, optional)
Запрос истории для выбранных биржи и инструмента

Запрос истории рынка для выбранных биржи и финансового инструмента. Данные имеют задержку в 15 минут, если запрос не авторизован. Для авторизованных клиентов задержка не применяется.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **symbol** | **String**| Тикер (Код финансового инструмента) | 
  **exchange** | [**Exchange**](.md)| Биржа | 
  **tf** | [**Duration**](.md)| Длительность таймфрейма в секундах или код (\&quot;D\&quot; - дни, \&quot;W\&quot; - недели, \&quot;M\&quot; - месяцы, \&quot;Y\&quot; - годы) | 
  **from** | **i32**| Начало отрезка времени (UTC) в формате Unix Time Seconds | 
  **to** | **i32**| Конец отрезка времени (UTC) в формате Unix Time Seconds | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **String**| Тикер (Код финансового инструмента) | 
 **exchange** | [**Exchange**](.md)| Биржа | 
 **tf** | [**Duration**](.md)| Длительность таймфрейма в секундах или код (\&quot;D\&quot; - дни, \&quot;W\&quot; - недели, \&quot;M\&quot; - месяцы, \&quot;Y\&quot; - годы) | 
 **from** | **i32**| Начало отрезка времени (UTC) в формате Unix Time Seconds | 
 **to** | **i32**| Конец отрезка времени (UTC) в формате Unix Time Seconds | 
 **untraded** | [**SchemaEnum**](.md)| Флаг для поиска данных по устаревшим или экспирированным инструментам. При использовании требуется точное совпадение тикера | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

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

Запрос биржевого стакана

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **seccode** | **String**| Инструмент | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **seccode** | **String**| Инструмент | 
 **depth** | **i32**| Глубина стакана. Стандартное и максимальное значение - 20 (20х20). | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**Orderbook**](orderbook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_quotes**
> Symbols dev_quotes(ctx, symbols, optional)
Получение информации о котировках для выбранных инструментов

Запрос информации о котировках для выбранных инструментов и бирж

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **symbols** | **String**| Принимает несколько пар биржа-тикер. Пары отделены запятыми. Биржа и тикер разделены двоеточием | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbols** | **String**| Принимает несколько пар биржа-тикер. Пары отделены запятыми. Биржа и тикер разделены двоеточием | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**Symbols**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_securities_futures**
> SymbolFutures dev_securities_futures(ctx, exchange, symbol, optional)
Получение котировки по ближайшему фьючерсу (код)

Запрос котировки по ближайшему фьючерсу (только по коду, без даты)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **symbol** | **String**| Тикер (Код финансового инструмента) | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **symbol** | **String**| Тикер (Код финансового инструмента) | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**SymbolFutures**](symbol_futures.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_securities_search**
> Securities dev_securities_search(ctx, query, optional)
Получение информации о торговых инструментах

Запрос информации о торговых инструментах

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **query** | **String**| Тикер (Код финансового инструмента) | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **query** | **String**| Тикер (Код финансового инструмента) | 
 **limit** | **i32**| Ограничение на количество выдаваемых результатов поиска | 
 **offset** | **i32**| Смещение начала выборки (для пагинации) | 
 **sector** | [**SchemaEnum**](.md)| Рынок на бирже | 
 **cficode** | **String**| Код финансового инструмента по стандарту ISO 10962 | 
 **exchange** | [**Exchange**](.md)| Биржа | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**Securities**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_securities_search_all_trades**
> Alltrades dev_securities_search_all_trades(ctx, exchange, symbol, optional)
Получение информации о всех сделках по ценным бумагам за сегодня

Запросить данные о всех сделках (лента) по ценным бумагам за сегодняшний день

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **symbol** | **String**| Тикер (Код финансового инструмента) | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **symbol** | **String**| Тикер (Код финансового инструмента) | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 
 **from** | **i32**| Начало отрезка времени (UTC) для фильтра результатов в формате Unix Time Seconds | 
 **to** | **i32**| Конец отрезка времени (UTC) для фильтра результатов в формате Unix Time Seconds | 
 **take** | **i32**| Количество загружаемых элементов | 
 **descending** | **bool**| Флаг загрузки элементов с конца списка | 
 **include_virtual_trades** | **bool**| Флаг загрузки виртуальных (индикативных) сделок, полученных из заявок на питерской бирже | 

### Return type

[**Alltrades**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_securities_search_exchange**
> Securities dev_securities_search_exchange(ctx, exchange, optional)
Получение информации о торговых инструментах на выбранной бирже

Запрос информации об инструментах на выбранной бирже

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

[**Securities**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_securities_search_exchange_code**
> Security dev_securities_search_exchange_code(ctx, exchange, symbol, optional)
Получение информации о выбранном финансовом инструменте

Запрос информации о выбранном финансовом инструменте на бирже

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
  **symbol** | **String**| Тикер (Код финансового инструмента) | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **symbol** | **String**| Тикер (Код финансового инструмента) | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

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

Получение ставок риска для маржинальной торговли.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **exchange** | [**Exchange**](.md)| Биржа | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exchange** | [**Exchange**](.md)| Биржа | 
 **ticker** | **String**| Тикер\\код инструмента, ISIN для облигаций | 
 **risk_category_id** | **String**| Id вашей (или той которая интересует) категории риска. Можно получить из запроса информации по клиенту или через кабинет клиента | 
 **search** | **String**| Часть Тикера\\кода инструмента, ISIN для облигаций. Вернет все совпадения, начинающиеся с  | 

### Return type

[**RiskRates**](riskRates.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

