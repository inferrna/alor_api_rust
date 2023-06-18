# {{classname}}

All URIs are relative to *https://apidev.alor.ru*

Method | HTTP request | Description
------------- | ------------- | -------------
[**command_api_v2clientordersactionslimit**](OrdersApi.md#command_api_v2clientordersactionslimit) | **POST** commandapi/warptrans/TRADE/v2/client/orders/actions/limit | Создание лимитной заявки
[**command_api_v2clientordersactionslimitput**](OrdersApi.md#command_api_v2clientordersactionslimitput) | **PUT** commandapi/warptrans/TRADE/v2/client/orders/actions/limit/{orderId} | Изменение лимитной заявки
[**command_api_v2clientordersactionsmarket**](OrdersApi.md#command_api_v2clientordersactionsmarket) | **POST** commandapi/warptrans/TRADE/v2/client/orders/actions/market | Создание рыночной заявки
[**command_api_v2clientordersactionsmarketput**](OrdersApi.md#command_api_v2clientordersactionsmarketput) | **PUT** commandapi/warptrans/TRADE/v2/client/orders/actions/market/{orderId} | Изменение рыночной заявки
[**command_api_v2clientordersdelete**](OrdersApi.md#command_api_v2clientordersdelete) | **DELETE** commandapi/warptrans/TRADE/v2/client/orders/{orderId} | Снятие заявки
[**v2clientordersactionsestimate**](OrdersApi.md#v2clientordersactionsestimate) | **POST** commandapi/warptrans/TRADE/v2/client/orders/estimate | Провести оценку одной заявки
[**v2clientordersactionsestimateall**](OrdersApi.md#v2clientordersactionsestimateall) | **POST** commandapi/warptrans/TRADE/v2/client/orders/estimate/all | Провести оценку нескольких заявок
[**v2clientordersactionsorder_id**](OrdersApi.md#v2clientordersactionsorder_id) | **DELETE** warptrans/{tradeServerCode}/v2/client/orders/{orderId} | Снятие стоп-заявки
[**v2clientordersactionsstop_loss**](OrdersApi.md#v2clientordersactionsstop_loss) | **POST** warptrans/{tradeServerCode}/v2/client/orders/actions/stopLoss | Создание стоп-лосс заявки
[**v2clientordersactionsstop_loss_limit**](OrdersApi.md#v2clientordersactionsstop_loss_limit) | **POST** warptrans/{tradeServerCode}/v2/client/orders/actions/stopLossLimit | Создание стоп-лосс лимит заявки
[**v2clientordersactionsstop_loss_limitorder_id**](OrdersApi.md#v2clientordersactionsstop_loss_limitorder_id) | **PUT** warptrans/{tradeServerCode}/v2/client/orders/actions/stopLossLimit/{orderId} | Изменение стоп-лосс лимит заявки
[**v2clientordersactionsstop_lossorder_id**](OrdersApi.md#v2clientordersactionsstop_lossorder_id) | **PUT** warptrans/{tradeServerCode}/v2/client/orders/actions/stopLoss/{orderId} | Изменение стоп-лосс заявки
[**v2clientordersactionstake_profit**](OrdersApi.md#v2clientordersactionstake_profit) | **POST** warptrans/{tradeServerCode}/v2/client/orders/actions/takeProfit | Создание стоп-заявки
[**v2clientordersactionstake_profit_limit**](OrdersApi.md#v2clientordersactionstake_profit_limit) | **POST** warptrans/{tradeServerCode}/v2/client/orders/actions/takeProfitLimit | Создание стоп-лимит заявки
[**v2clientordersactionstake_profit_limitorder_id**](OrdersApi.md#v2clientordersactionstake_profit_limitorder_id) | **PUT** warptrans/{tradeServerCode}/v2/client/orders/actions/takeProfitLimit/{orderId} | Изменение стоп-лимит заявки
[**v2clientordersactionstake_profitorder_id**](OrdersApi.md#v2clientordersactionstake_profitorder_id) | **PUT** warptrans/{tradeServerCode}/v2/client/orders/actions/takeProfit/{orderId} | Изменение стоп-заявки

# **command_api_v2clientordersactionslimit**
> OrdersActionsLimitMarketCommandApi command_api_v2clientordersactionslimit(ctx, body, x_alor_reqid)
Создание лимитной заявки

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsLimitTv**](BodyrequestOrdersActionsLimitTv.md)| Тело заявки | 
  **x_alor_reqid** | **String**| Через точку с запятой портфель и уникальный идентификатор запроса &#x60;&#x60;portfolio;uid&#x60;&#x60;. В качестве идентификатора запроса требуется уникальная случайная строка. Если уже приходил запрос с таким идентификатором, то заявка не будет исполнена повторно, а в качестве ответа будет возвращена копия ответа на предыдущий запрос с таким значением идентификатора. | 

### Return type

[**OrdersActionsLimitMarketCommandApi**](orders_actions_LimitMarket_CommandAPI.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **command_api_v2clientordersactionslimitput**
> OrdersActionsLimitMarket command_api_v2clientordersactionslimitput(ctx, body, order_id, x_alor_reqid)
Изменение лимитной заявки

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsLimitTVput**](BodyrequestOrdersActionsLimitTVput.md)| Тело заявки | 
  **order_id** | **String**| Идентификатор заявки | 
  **x_alor_reqid** | **String**| Через точку с запятой портфолио и уникальный идентификатор запроса &#x60;&#x60;portfolio;uid&#x60;&#x60;. В качестве идентификатора запроса требуется уникальная случайная строка из цифр. Если уже приходил запрос с таким идентификатором, то заявка не будет исполнена повторно, а в качестве ответа будет возвращена копия ответа на первый запрос с таким значением идентификатора | 

### Return type

[**OrdersActionsLimitMarket**](orders_actions_LimitMarket.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **command_api_v2clientordersactionsmarket**
> OrdersActionsLimitMarketCommandApi command_api_v2clientordersactionsmarket(ctx, body, x_alor_reqid)
Создание рыночной заявки

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsMarketTv**](BodyrequestOrdersActionsMarketTv.md)| Тело заявки | 
  **x_alor_reqid** | **String**| Через точку с запятой портфель и уникальный идентификатор запроса &#x60;&#x60;portfolio;uid&#x60;&#x60;. В качестве идентификатора запроса требуется уникальная случайная строка. Если уже приходил запрос с таким идентификатором, то заявка не будет исполнена повторно, а в качестве ответа будет возвращена копия ответа на предыдущий запрос с таким значением идентификатора. | 

### Return type

[**OrdersActionsLimitMarketCommandApi**](orders_actions_LimitMarket_CommandAPI.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **command_api_v2clientordersactionsmarketput**
> OrdersActionsLimitMarket command_api_v2clientordersactionsmarketput(ctx, body, order_id, x_alor_reqid)
Изменение рыночной заявки

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsMarketTVput**](BodyrequestOrdersActionsMarketTVput.md)| Тело заявки | 
  **order_id** | **String**| Идентификатор заявки | 
  **x_alor_reqid** | **String**| Через точку с запятой портфолио и уникальный идентификатор запроса &#x60;&#x60;portfolio;uid&#x60;&#x60;. В качестве идентификатора запроса требуется уникальная случайная строка из цифр. Если уже приходил запрос с таким идентификатором, то заявка не будет исполнена повторно, а в качестве ответа будет возвращена копия ответа на первый запрос с таким значением идентификатора | 

### Return type

[**OrdersActionsLimitMarket**](orders_actions_LimitMarket.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **command_api_v2clientordersdelete**
> OrdersActionsDeleteOrderIdCommandApi command_api_v2clientordersdelete(ctx, order_id, portfolio, exchange, stop, optional)
Снятие заявки

Снятие заявки с указанным идентификатором

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **order_id** | **i32**| Идентификатор заявки | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
  **exchange** | [**Exchange**](.md)| Биржа | 
  **stop** | [**SchemaEnum**](.md)| Является стоп-заявкой? | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **order_id** | **i32**| Идентификатор заявки | 
 **portfolio** | **String**| Идентификатор клиентского портфеля | 
 **exchange** | [**Exchange**](.md)| Биржа | 
 **stop** | [**SchemaEnum**](.md)| Является стоп-заявкой? | 
 **json_response** | [**SchemaEnum**](.md)| Ответ в формате JSON. В виде отдельного параметра для обратной совместимости. | 
 **format** | [**JsonFormat**](.md)| Формат возвращаемого сервером JSON | 

### Return type

**OrdersActionsDeleteOrderIdCommandApi**

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2clientordersactionsestimate**
> EstimateOrderModel v2clientordersactionsestimate(optional)
Провести оценку одной заявки

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**EstimateOrderViewModel**](EstimateOrderViewModel.md)| Параметры заявки | 

### Return type

[**EstimateOrderModel**](estimateOrderModel.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/_*+json, application/json, application/json-patch+json, text/json
 - **Accept**: application/json, text/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2clientordersactionsestimateall**
> Vec<EstimateOrderModel> v2clientordersactionsestimateall(optional)
Провести оценку нескольких заявок

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**Vec&lt;EstimateOrderViewModel&gt;**](estimateOrderViewModel.md)| Список параметров заявок | 

### Return type

[**Vec<EstimateOrderModel>**](estimateOrderModel.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/_*+json, application/json, application/json-patch+json, text/json
 - **Accept**: application/json, text/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2clientordersactionsorder_id**
> OrdersActionsDeleteOrderId v2clientordersactionsorder_id(ctx, trade_server_code, order_id, portfolio, stop, x_alor_reqid)
Снятие стоп-заявки

Снятие стоп-заявки с указанным идентификатором

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **trade_server_code** | **String**| Код торгового сервера | 
  **order_id** | **i32**| Идентификатор заявки | 
  **portfolio** | **String**| Идентификатор клиентского портфеля | 
  **stop** | **bool**| Является стоп-заявкой? | 
  **x_alor_reqid** | **String**| Требуется уникальная случайная строка в качестве идентификатора запроса. Если уже приходил запрос с таким идентификатором, то заявка не будет исполнена повторно, а в качестве ответа будет возвращена копия ответа на первый запрос с таким значением идентификатора | 

### Return type

**OrdersActionsDeleteOrderId**

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, string

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2clientordersactionsstop_loss**
> OrdersActionsStopProfitLoss v2clientordersactionsstop_loss(ctx, body, trade_server_code, x_alor_reqid)
Создание стоп-лосс заявки

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsStop**](BodyrequestOrdersActionsStop.md)| Тело заявки | 
  **trade_server_code** | **String**| Код торгового сервера | 
  **x_alor_reqid** | **String**| Требуется уникальная случайная строка в качестве идентификатора запроса. Если уже приходил запрос с таким идентификатором, то заявка не будет исполнена повторно, а в качестве ответа будет возвращена копия ответа на первый запрос с таким значением идентификатора | 

### Return type

[**OrdersActionsStopProfitLoss**](orders_actions_StopProfitLoss.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2clientordersactionsstop_loss_limit**
> OrdersActionsStopProfitLoss v2clientordersactionsstop_loss_limit(ctx, body, trade_server_code, x_alor_reqid)
Создание стоп-лосс лимит заявки

Создание стоп-лосс лимит заявки

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsStoplimit**](BodyrequestOrdersActionsStoplimit.md)| Тело заявки | 
  **trade_server_code** | **String**| Код торгового сервера | 
  **x_alor_reqid** | **String**| Требуется уникальная случайная строка в качестве идентификатора запроса. Если уже приходил запрос с таким идентификатором, то заявка не будет исполнена повторно, а в качестве ответа будет возвращена копия ответа на первый запрос с таким значением идентификатора | 

### Return type

[**OrdersActionsStopProfitLoss**](orders_actions_StopProfitLoss.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2clientordersactionsstop_loss_limitorder_id**
> OrdersActionsStopProfitLoss v2clientordersactionsstop_loss_limitorder_id(ctx, body, trade_server_code, order_id, x_alor_reqid)
Изменение стоп-лосс лимит заявки

Изменение стоп-лосс лимит заявки с указанным номером

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsStoplimit**](BodyrequestOrdersActionsStoplimit.md)| Тело заявки | 
  **trade_server_code** | **String**| Код торгового сервера | 
  **order_id** | **i32**| Идентификатор заявки | 
  **x_alor_reqid** | **String**| Требуется уникальная случайная строка в качестве идентификатора запроса. Если уже приходил запрос с таким идентификатором, то заявка не будет исполнена повторно, а в качестве ответа будет возвращена копия ответа на первый запрос с таким значением идентификатора | 

### Return type

[**OrdersActionsStopProfitLoss**](orders_actions_StopProfitLoss.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, string

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2clientordersactionsstop_lossorder_id**
> OrdersActionsStopProfitLoss v2clientordersactionsstop_lossorder_id(ctx, body, trade_server_code, order_id, x_alor_reqid)
Изменение стоп-лосс заявки

Изменение стоп-лосс заявки с указанным номером

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsStop**](BodyrequestOrdersActionsStop.md)| Тело заявки | 
  **trade_server_code** | **String**| Код торгового сервера | 
  **order_id** | **i32**| Идентификатор заявки | 
  **x_alor_reqid** | **String**| Требуется уникальная случайная строка в качестве идентификатора запроса. Если уже приходил запрос с таким идентификатором, то заявка не будет исполнена повторно, а в качестве ответа будет возвращена копия ответа на первый запрос с таким значением идентификатора | 

### Return type

[**OrdersActionsStopProfitLoss**](orders_actions_StopProfitLoss.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, string

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2clientordersactionstake_profit**
> OrdersActionsStopProfitLoss v2clientordersactionstake_profit(ctx, body, trade_server_code, x_alor_reqid)
Создание стоп-заявки

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsStop**](BodyrequestOrdersActionsStop.md)| Тело заявки | 
  **trade_server_code** | **String**| Код торгового сервера | 
  **x_alor_reqid** | **String**| Требуется уникальная случайная строка в качестве идентификатора запроса. Если уже приходил запрос с таким идентификатором, то заявка не будет исполнена повторно, а в качестве ответа будет возвращена копия ответа на первый запрос с таким значением идентификатора | 

### Return type

[**OrdersActionsStopProfitLoss**](orders_actions_StopProfitLoss.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2clientordersactionstake_profit_limit**
> OrdersActionsStopProfitLoss v2clientordersactionstake_profit_limit(ctx, body, trade_server_code, x_alor_reqid)
Создание стоп-лимит заявки

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsStoplimit**](BodyrequestOrdersActionsStoplimit.md)| Тело заявки | 
  **trade_server_code** | **String**| Код торгового сервера | 
  **x_alor_reqid** | **String**| Требуется уникальная случайная строка в качестве идентификатора запроса. Если уже приходил запрос с таким идентификатором, то заявка не будет исполнена повторно, а в качестве ответа будет возвращена копия ответа на первый запрос с таким значением идентификатора | 

### Return type

[**OrdersActionsStopProfitLoss**](orders_actions_StopProfitLoss.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2clientordersactionstake_profit_limitorder_id**
> OrdersActionsStopProfitLoss v2clientordersactionstake_profit_limitorder_id(ctx, body, trade_server_code, order_id, x_alor_reqid)
Изменение стоп-лимит заявки

Изменение стоп-лимит заявки с указанным номером

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsStoplimit**](BodyrequestOrdersActionsStoplimit.md)| Тело заявки | 
  **trade_server_code** | **String**| Код торгового сервера | 
  **order_id** | **i32**| Идентификатор заявки | 
  **x_alor_reqid** | **String**| Требуется уникальная случайная строка в качестве идентификатора запроса. Если уже приходил запрос с таким идентификатором, то заявка не будет исполнена повторно, а в качестве ответа будет возвращена копия ответа на первый запрос с таким значением идентификатора | 

### Return type

[**OrdersActionsStopProfitLoss**](orders_actions_StopProfitLoss.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, string

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2clientordersactionstake_profitorder_id**
> OrdersActionsStopProfitLoss v2clientordersactionstake_profitorder_id(ctx, body, trade_server_code, order_id, x_alor_reqid)
Изменение стоп-заявки

Изменение стоп-заявки с указанным номером

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsStop**](BodyrequestOrdersActionsStop.md)| Тело заявки | 
  **trade_server_code** | **String**| Код торгового сервера | 
  **order_id** | **i32**| Идентификатор заявки | 
  **x_alor_reqid** | **String**| Требуется уникальная случайная строка в качестве идентификатора запроса. Если уже приходил запрос с таким идентификатором, то заявка не будет исполнена повторно, а в качестве ответа будет возвращена копия ответа на первый запрос с таким значением идентификатора | 

### Return type

[**OrdersActionsStopProfitLoss**](orders_actions_StopProfitLoss.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, string

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

