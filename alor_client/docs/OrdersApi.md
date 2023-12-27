# {{classname}}

All URIs are relative to *https://apidev.alor.ru*

Method | HTTP request | Description
------------- | ------------- | -------------
[**command_api_v2clientordersactionslimit**](OrdersApi.md#command_api_v2clientordersactionslimit) | **POST** commandapi/warptrans/TRADE/v2/client/orders/actions/limit | Создание лимитной заявки
[**command_api_v2clientordersactionslimitput**](OrdersApi.md#command_api_v2clientordersactionslimitput) | **PUT** commandapi/warptrans/TRADE/v2/client/orders/actions/limit/{orderId} | Изменение лимитной заявки
[**command_api_v2clientordersactionsmarket**](OrdersApi.md#command_api_v2clientordersactionsmarket) | **POST** commandapi/warptrans/TRADE/v2/client/orders/actions/market | Создание рыночной заявки
[**command_api_v2clientordersactionsmarketput**](OrdersApi.md#command_api_v2clientordersactionsmarketput) | **PUT** commandapi/warptrans/TRADE/v2/client/orders/actions/market/{orderId} | Изменение рыночной заявки
[**command_api_warp_v2clientordersdelete**](OrdersApi.md#command_api_warp_v2clientordersdelete) | **DELETE** commandapi/warptrans/TRADE/v2/client/orders/{orderId} | Снятие заявки
[**dev_get_all_orders**](OrdersApi.md#dev_get_all_orders) | **GET** md/v2/clients/{exchange}/{portfolio}/orders | Получение информации о всех заявках
[**dev_get_one_order**](OrdersApi.md#dev_get_one_order) | **GET** md/v2/clients/{exchange}/{portfolio}/orders/{orderId} | Получение информации о выбранной заявке
[**v2clientordersactionsestimate**](OrdersApi.md#v2clientordersactionsestimate) | **POST** commandapi/warptrans/TRADE/v2/client/orders/estimate | Провести оценку одной заявки
[**v2clientordersactionsestimateall**](OrdersApi.md#v2clientordersactionsestimateall) | **POST** commandapi/warptrans/TRADE/v2/client/orders/estimate/all | Провести оценку нескольких заявок

# **command_api_v2clientordersactionslimit**
> OrdersActionsLimitMarketCommandApi command_api_v2clientordersactionslimit(ctx, body, x_alor_reqid)
Создание лимитной заявки

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос создаёт на бирже новую заявку на покупку или продажу торгового инструмента по указанной в теле запроса цене. 

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

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос изменяет характеристики ранее поданной лимитной заявки с указанным номером 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**BodyrequestOrdersActionsLimitTv**](BodyrequestOrdersActionsLimitTv.md)| Тело заявки | 
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

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос создаёт на бирже новую заявку на покупку или продажу торгового инструмента по рыночной цене. 

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

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Запрос изменяет характеристики ранее поданной рыночной заявки с указанным номером 

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

# **command_api_warp_v2clientordersdelete**
> String command_api_warp_v2clientordersdelete(ctx, order_id, portfolio, exchange, stop, optional)
Снятие заявки

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`.  Снятие заявки с указанным идентификатором 

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

# **v2clientordersactionsestimate**
> EstimateOrderModel v2clientordersactionsestimate(ctx, optional)
Провести оценку одной заявки

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**EstimateOrderViewModel**](EstimateOrderViewModel.md)| Параметры заявки | 

### Return type

[**EstimateOrderModel**](estimateOrderModel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/_*+json, application/json, application/json-patch+json, text/json
 - **Accept**: application/json, text/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2clientordersactionsestimateall**
> Vec<EstimateOrderModel> v2clientordersactionsestimateall(ctx, optional)
Провести оценку нескольких заявок

**Запрос нельзя выполнить анонимно**. Для авторизации запроса добавьте заголовок `Authorization` со значением `Bearer <ваш JWT>`. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**Vec&lt;EstimateOrderViewModel&gt;**](estimateOrderViewModel.md)| Список параметров заявок | 

### Return type

[**Vec<EstimateOrderModel>**](estimateOrderModel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: application/_*+json, application/json, application/json-patch+json, text/json
 - **Accept**: application/json, text/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

