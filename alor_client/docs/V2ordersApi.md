# {{classname}}

All URIs are relative to *https://apidev.alor.ru*

Method | HTTP request | Description
------------- | ------------- | -------------
[**command_api_v2clientordersactionsstop**](V2ordersApi.md#command_api_v2clientordersactionsstop) | **POST** commandapi/warptrans/TRADE/v2/client/orders/actions/stop | Создание стоп заявки
[**command_api_v2clientordersactionsstop_limit**](V2ordersApi.md#command_api_v2clientordersactionsstop_limit) | **POST** commandapi/warptrans/TRADE/v2/client/orders/actions/stopLimit | Создание стоп-лимитной заявки
[**command_api_v2clientordersactionsstop_limitstop_order_id**](V2ordersApi.md#command_api_v2clientordersactionsstop_limitstop_order_id) | **PUT** commandapi/warptrans/TRADE/v2/client/orders/actions/stopLimit/{stopOrderId} | Изменение стоп-лимитной заявки
[**command_api_warp_v2clientordersdelete**](V2ordersApi.md#command_api_warp_v2clientordersdelete) | **DELETE** commandapi/warptrans/TRADE/v2/client/orders/{orderId}/ | Снятие заявки
[**dev_get_all_stop_orders**](V2ordersApi.md#dev_get_all_stop_orders) | **GET** md/v2/clients/{exchange}/{portfolio}/stoporders | Получение информации о стоп-заявках
[**dev_get_one_stop_order**](V2ordersApi.md#dev_get_one_stop_order) | **GET** md/v2/clients/{exchange}/{portfolio}/stoporders/{orderId} | Получение информации о выбранной стоп-заявке

# **command_api_v2clientordersactionsstop**
> OrdersActionsLimitMarketCommandApi command_api_v2clientordersactionsstop(ctx, body, x_alor_reqid)
Создание стоп заявки

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsStopMarketTvWarp**](BodyrequestOrdersActionsStopMarketTvWarp.md)| Тело заявки | 
  **x_alor_reqid** | **String**|  | 

### Return type

[**OrdersActionsLimitMarketCommandApi**](orders_actions_LimitMarket_CommandAPI.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **command_api_v2clientordersactionsstop_limit**
> OrdersActionsLimitMarketCommandApi command_api_v2clientordersactionsstop_limit(ctx, body, x_alor_reqid)
Создание стоп-лимитной заявки

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsStopLimitTvWarp**](BodyrequestOrdersActionsStopLimitTvWarp.md)| Тело заявки | 
  **x_alor_reqid** | **String**|  | 

### Return type

[**OrdersActionsLimitMarketCommandApi**](orders_actions_LimitMarket_CommandAPI.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **command_api_v2clientordersactionsstop_limitstop_order_id**
> OrdersActionsLimitMarketCommandApi command_api_v2clientordersactionsstop_limitstop_order_id(ctx, body, x_alor_reqid, stop_order_id)
Изменение стоп-лимитной заявки

Изменение стоп-лимитной заявки

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsStopLimitTvWarp**](BodyrequestOrdersActionsStopLimitTvWarp.md)| Тело заявки | 
  **x_alor_reqid** | **String**|  | 
  **stop_order_id** | **i64**|  | 

### Return type

[**OrdersActionsLimitMarketCommandApi**](orders_actions_LimitMarket_CommandAPI.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **command_api_warp_v2clientordersdelete**
> String command_api_warp_v2clientordersdelete(ctx, order_id, portfolio, exchange, stop, optional)
Снятие заявки

Снятие заявки с указанным идентификатором

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **order_id** | **i64**|  | 
  **portfolio** | **String**|  | 
  **exchange** | [**Exchange**](.md)|  | 
  **stop** | **bool**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **order_id** | **i64**|  | 
 **portfolio** | **String**|  | 
 **exchange** | [**Exchange**](.md)|  | 
 **stop** | **bool**|  | 
 **json_response** | **bool**|  | 
 **format** | [**Format**](.md)|  | 

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dev_get_all_stop_orders**
> Vec<StoporderWarp> dev_get_all_stop_orders(ctx, exchange, portfolio, optional)
Получение информации о стоп-заявках

Запрос информации о всех стоп-заявках

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

# **dev_get_one_stop_order**
> StoporderWarp dev_get_one_stop_order(ctx, exchange, portfolio, order_id, optional)
Получение информации о выбранной стоп-заявке

Запрос информации о выбранной стоп-заявке

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

