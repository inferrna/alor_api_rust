# BodyrequestOrdersActionsStopLimitTvWarp

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**condition** | **Condition** |  | [default to null]
**iceberg_fixed** | **i32** | Видимая постоянная часть айсберг-заявки в лотах | [default to null]
**iceberg_variance** | **Decimal** | Амплитуда отклонения (в % от icebergFixed) случайной надбавки к видимой части айсберг-заявки. Только срочный рынок | [default to null]
**instrument** | [***BodyrequestOrdersActionsStopLimitTvWarpInstrument**](bodyrequest_OrdersActionsStopLimitTVWarp_instrument.md) |  | [default to null]
**price** | **Decimal** | Цена выставления лимитной заявки | [default to null]
**quantity** | **i32** | Количество (лоты) | [default to null]
**side** | **Side** |  | [default to null]
**stop_end_unix_time** | **i64** | Срок действия (UTC) в формате Unix Time seconds | [default to null]
**time_in_force** | **LifePolicy** |  | [default to null]
**trigger_price** | **Decimal** | Цена срабатывания | [default to null]
**user** | [***BodyrequestOrdersActionsStopLimitTvWarpUser**](bodyrequest_OrdersActionsStopLimitTVWarp_user.md) |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

