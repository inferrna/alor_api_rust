# BodyrequestOrdersActionsLimitTv

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comment** | **String** | Пользовательский комментарий к заявке | [default to null]
**iceberg_fixed** | **i32** | Видимая постоянная часть айсберг-заявки в лотах | [default to null]
**iceberg_variance** | **Decimal** | Амплитуда отклонения (в % от icebergFixed) случайной надбавки к видимой части айсберг-заявки. Только срочный рынок | [default to null]
**instrument** | [***BodyrequestOrdersActionsLimitTvInstrument**](bodyrequest_OrdersActionsLimitTV_instrument.md) |  | [default to null]
**price** | **Decimal** | Цена | [default to null]
**quantity** | **i32** | Количество (лоты) | [default to null]
**side** | **Side** |  | [default to null]
**time_in_force** | **LifePolicy** |  | [default to null]
**rtype** | **String** | Тип заявки | [default to null]
**user** | [***BodyrequestOrdersActionsLimitTvUser**](bodyrequest_OrdersActionsLimitTV_user.md) |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

