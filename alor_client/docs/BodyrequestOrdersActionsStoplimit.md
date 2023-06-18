# BodyrequestOrdersActionsStoplimit

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**instrument** | [***BodyrequestOrdersActionsLimitInstrument**](bodyrequest_OrdersActionsLimit_Instrument.md) |  | [default to null]
**order_end_unix_time** | **i32** | Время (UTC) завершения сделки в формате Unix Time seconds | [default to null]
**price** | **i32** | Цена | [default to null]
**quantity** | **i32** | Количество | [default to null]
**side** | **Operation** |  | [default to null]
**trigger_price** | **i32** | Стоп-цена | [default to null]
**user** | [***BodyrequestOrdersActionsLimitUser**](bodyrequest_OrdersActionsLimit_User.md) |  | [default to null]
**iceberg_fixed** | [***Decimal**](BigDecimal.md) | Видимая постоянная часть айсберг-заявки в лотах | [default to null]
**iceberg_variance** | [***Decimal**](BigDecimal.md) | Амплитуда отклонения (в % от icebergFixed) случайной надбавки к видимой части айсберг-заявки. Только срочный рынок | [default to null]
**time_in_force** | **LifePolicy** |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

