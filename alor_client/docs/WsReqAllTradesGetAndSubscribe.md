# WsReqAllTradesGetAndSubscribe

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** | Тикер | [default to null]
**depth** | **i32** | Если указать, то перед актуальными данными придут данные о последних N сделках. | [default to null]
**exchange** | **Exchange** |  | [default to null]
**format** | **Format** |  | [default to null]
**guid** | **String** | Уникальный идентификатор сообщений создаваемой подписки. Все входящие сообщения, соответствующие этой подписке, будут иметь такое значение поля guid. | [default to null]
**include_virtual_trades** | **bool** | Указывает, нужно ли отправлять виртуальные (индикативные) сделки | [default to null]
**opcode** | **OpcodeEnum** |  | [default to null]
**token** | **String** | JWT токен для авторизации | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

