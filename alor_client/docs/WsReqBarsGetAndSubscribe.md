# WsReqBarsGetAndSubscribe

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** | Тикер | [default to null]
**delayed** | **String** | Данные c задержкой в 15 минут. Для авторизованых клиентов задержка не применяется. | [default to null]
**exchange** | **Exchange** |  | [default to null]
**format** | **JsonFormat** |  | [default to null]
**from** | **i64** | Дата и время (UTC) для первой запрашиваемой свечи | [default to null]
**guid** | **String** | Уникальный идентификатор сообщений создаваемой подписки. Все входящие сообщения, соответствующие этой подписке, будут иметь такое значение поля guid. | [default to null]
**opcode** | **OpcodeEnum** |  | [default to null]
**tf** | **Duration** |  | [default to null]
**token** | **String** | JWT токен для авторизации | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

