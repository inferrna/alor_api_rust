# BodyrequestOrdersActionsStopMarketTvWarp

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activate** | **bool** | Флаг указывает, создать активную заявку, или не активную. Не активная заявка отображается в системе, но не участвует в процессе выставления на биржу, пока не станет активной. Данный флаг необходим при создании группы заявок с типом &#x27;TriggerBracketOrders&#x27;  | [default to true]
**condition** | **Condition** |  | [default to null]
**instrument** | [***BodyrequestOrdersActionsStopLimitTvWarpInstrument**](bodyrequest_OrdersActionsStopLimitTVWarp_instrument.md) |  | [default to null]
**quantity** | **i32** | Количество (лоты) | [default to null]
**side** | **Side** |  | [default to null]
**stop_end_unix_time** | **i64** | Срок действия (UTC) в формате Unix Time seconds | [default to null]
**trigger_price** | **Decimal** | Цена срабатывания | [default to null]
**user** | [***BodyrequestOrdersActionsStopLimitTvWarpUser**](bodyrequest_OrdersActionsStopLimitTVWarp_user.md) |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

