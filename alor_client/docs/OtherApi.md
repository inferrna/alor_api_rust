# {{classname}}

All URIs are relative to *https://apidev.alor.ru*

Method | HTTP request | Description
------------- | ------------- | -------------
[**local_time**](OtherApi.md#local_time) | **GET** md/v2/time | Запрос текущего UTC времени в формате Unix

# **local_time**
> i64 local_time(ctx, )
Запрос текущего UTC времени в формате Unix

**Запрос может быть выполнен без авторизации**. При отправке анонимного запроса вернутся данные, бывшие актуальными 15 минут назад.  Запрос текущего UTC времени в формате Unix Time Seconds. Если этот запрос выполнен без авторизации, то будет возвращено время, которое было 15 минут назад. 

### Required Parameters
This endpoint does not need any parameter.

### Return type

**i64**

### Authorization

[bearerAuth](../README.md#bearerAuth), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

