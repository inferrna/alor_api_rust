# BodyrequestOrdersActionsLimitTVput

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**iceberg_fixed** | [***Decimal**](BigDecimal.md) | Видимая постоянная часть айсберг-заявки в лотах | [default to null]
**iceberg_variance** | [***Decimal**](BigDecimal.md) | Амплитуда отклонения (в % от icebergFixed) случайной надбавки к видимой части айсберг-заявки. Только срочный рынок | [default to null]
**id** | **i64** | Идентификатор заявки | [default to null]
**instrument** | [***BodyrequestOrdersActionsLimitTvInstrument**](bodyrequest_OrdersActionsLimitTV_instrument.md) |  | [default to null]
**price** | [***Decimal**](BigDecimal.md) | Цена | [default to null]
**quantity** | **i32** | Количество | [default to null]
**side** | **Operation** |  | [default to null]
**time_in_force** | **LifePolicy** |  | [default to null]
**rtype** | **String** | Тип заявки | [default to null]
**user** | [***BodyrequestOrdersActionsLimitTvUser**](bodyrequest_OrdersActionsLimitTV_user.md) |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

