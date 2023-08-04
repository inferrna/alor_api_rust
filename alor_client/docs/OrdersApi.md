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

# **command_api_v2clientordersactionslimit**
> OrdersActionsLimitMarketCommandApi command_api_v2clientordersactionslimit(ctx, body, x_alor_reqid)
Создание лимитной заявки

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsLimitTv**](BodyrequestOrdersActionsLimitTv.md)| Тело заявки | 
  **x_alor_reqid** | **String**|  | 

### Return type

[**OrdersActionsLimitMarketCommandApi**](orders_actions_LimitMarket_CommandAPI.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **command_api_v2clientordersactionslimitput**
> OrdersActionsLimitMarket command_api_v2clientordersactionslimitput(ctx, body, x_alor_reqid, order_id)
Изменение лимитной заявки

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsLimitTVput**](BodyrequestOrdersActionsLimitTVput.md)| Тело заявки | 
  **x_alor_reqid** | **String**|  | 
  **order_id** | **i64**|  | 

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
  **x_alor_reqid** | **String**|  | 

### Return type

[**OrdersActionsLimitMarketCommandApi**](orders_actions_LimitMarket_CommandAPI.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **command_api_v2clientordersactionsmarketput**
> OrdersActionsLimitMarket command_api_v2clientordersactionsmarketput(ctx, body, x_alor_reqid, order_id)
Изменение рыночной заявки

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsMarketTVput**](BodyrequestOrdersActionsMarketTVput.md)| Тело заявки | 
  **x_alor_reqid** | **String**|  | 
  **order_id** | **i64**|  | 

### Return type

[**OrdersActionsLimitMarket**](orders_actions_LimitMarket.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **command_api_v2clientordersdelete**
> String command_api_v2clientordersdelete(ctx, order_id, portfolio, exchange, stop, optional)
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

