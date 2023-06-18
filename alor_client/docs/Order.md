# Order

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**broker_symbol** | **String** | Пара биржа-Тикер | [default to null]
**end_time** | **String** | Время завершения (UTC) | [default to null]
**exchange** | **Exchange** |  | [default to null]
**existing** | **bool** | True - для данных из \&quot;снепшота\&quot;, то есть из истории. False - для новых событий | [default to null]
**filled** | [***Decimal**](BigDecimal.md) | Количество исполненных (лоты) | [default to null]
**filled_qty_batch** | [***Decimal**](BigDecimal.md) | Количество исполненных (лоты) | [default to null]
**filled_qty_units** | [***Decimal**](BigDecimal.md) | Количество исполненных (штуки) | [default to null]
**id** | **String** | Уникальный идентификатор заявки | [default to null]
**price** | [***Decimal**](BigDecimal.md) | Цена | [default to null]
**qty** | [***Decimal**](BigDecimal.md) | Количество (лоты) | [default to null]
**qty_batch** | [***Decimal**](BigDecimal.md) | Количество (лоты) | [default to null]
**qty_units** | [***Decimal**](BigDecimal.md) | Количество (штуки) | [default to null]
**side** | **Operation** |  | [default to null]
**status** | **OrderStatus** |  | [default to null]
**symbol** | **String** | Тикер (Код финансового инструмента) | [default to null]
**trans_time** | **String** | Время выставления (UTC) | [default to null]
**rtype** | **OrderType** |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
