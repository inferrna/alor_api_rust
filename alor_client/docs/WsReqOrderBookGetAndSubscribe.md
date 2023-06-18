# WsReqOrderBookGetAndSubscribe

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** | Тикер | [default to null]
**depth** | **i32** | Глубина стакана. Стандартное и максимальное значение - 20 (20х20). | [default to null]
**exchange** | **Exchange** |  | [default to null]
**format** | **JsonFormat** |  | [default to null]
**guid** | **String** | Уникальный идентификатор сообщений создаваемой подписки. Все входящие сообщения, соответствующие этой подписке, будут иметь такое значение поля guid. | [default to null]
**opcode** | **OpcodeEnum** |  | [default to null]
**token** | **String** | JWT токен для авторизации | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
