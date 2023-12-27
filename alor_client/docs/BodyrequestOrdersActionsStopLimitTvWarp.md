# BodyrequestOrdersActionsStopLimitTvWarp

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activate** | **bool** | Флаг указывает, создать активную заявку, или не активную. Не активная заявка отображается в системе, но не участвует в процессе выставления на биржу, пока не станет активной. Данный флаг необходим при создании группы заявок с типом &#x60;TriggerBracketOrders&#x60;  | [default to true]
**comment** | **String** | Пользовательский комментарий к заявке | [default to null]
**condition** | **Condition** |  | [default to null]
**iceberg_fixed** | **i32** | Видимая постоянная часть айсберг-заявки в лотах | [default to null]
**iceberg_variance** | **Decimal** | Амплитуда отклонения (в % от icebergFixed) случайной надбавки к видимой части айсберг-заявки. Только срочный рынок | [default to null]
**instrument** | [***BodyrequestOrdersActionsStopLimitTvWarpInstrument**](bodyrequest_OrdersActionsStopLimitTVWarp_instrument.md) |  | [default to null]
**price** | **Decimal** | Цена выставления стоп-лимитной заявки | [default to null]
**quantity** | **i32** | Количество (лоты) | [default to null]
**side** | **Side** |  | [default to null]
**stop_end_unix_time** | **i64** | Срок действия (UTC) в формате Unix Time seconds | [default to null]
**time_in_force** | **LifePolicy** |  | [default to null]
**trigger_price** | **Decimal** | Стоп-цена | [default to null]
**user** | [***BodyrequestOrdersActionsStopLimitTvWarpUser**](bodyrequest_OrdersActionsStopLimitTVWarp_user.md) |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

