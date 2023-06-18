# BodyrequestOrdersActionsStopLimitTvWarp

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**condition** | **String** | Условие срабатывания more/less | [default to null]
**iceberg_fixed** | [***Decimal**](BigDecimal.md) | Видимая постоянная часть айсберг-заявки в лотах | [default to null]
**iceberg_variance** | [***Decimal**](BigDecimal.md) | Амплитуда отклонения (в % от icebergFixed) случайной надбавки к видимой части айсберг-заявки. Только срочный рынок | [default to null]
**instrument** | [***BodyrequestOrdersActionsStopLimitTvWarpInstrument**](bodyrequest_OrdersActionsStopLimitTVWarp_instrument.md) |  | [default to null]
**price** | [***Decimal**](BigDecimal.md) | Цена выставления лимитной заявки | [default to null]
**quantity** | **i32** | Количество (лоты) | [default to null]
**side** | **Operation** |  | [default to null]
**stop_end_unix_time** | [***Decimal**](BigDecimal.md) | Срок действия (UTC) в формате Unix Time seconds | [default to null]
**time_in_force** | **LifePolicy** |  | [default to null]
**trigger_price** | [***Decimal**](BigDecimal.md) | Цена срабатывания | [default to null]
**user** | [***BodyrequestOrdersActionsStopLimitTvWarpUser**](bodyrequest_OrdersActionsStopLimitTVWarp_user.md) |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

