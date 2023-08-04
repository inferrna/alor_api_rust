# Order

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**broker_symbol** | **String** | Пара биржа-Тикер | [default to null]
**end_time** | **DateTime<Utc>** | Время завершения (UTC) | [default to null]
**exchange** | **Exchange** |  | [default to null]
**existing** | **bool** | True - для данных из \&quot;снепшота\&quot;, то есть из истории. False - для новых событий | [default to null]
**filled** | **Decimal** | Количество исполненных (лоты) | [default to null]
**filled_qty_batch** | **Decimal** | Количество исполненных (лоты) | [default to null]
**filled_qty_units** | **Decimal** | Количество исполненных (штуки) | [default to null]
**id** | **String** | Уникальный идентификатор заявки | [default to null]
**price** | **Decimal** | Цена | [default to null]
**qty** | **Decimal** | Количество (лоты) | [default to null]
**qty_batch** | **Decimal** | Количество (лоты) | [default to null]
**qty_units** | **i32** | Количество (штуки) | [default to null]
**side** | **Side** |  | [default to null]
**status** | **OrderStatus** |  | [default to null]
**symbol** | **String** | Тикер (Код финансового инструмента) | [default to null]
**trans_time** | **DateTime<Utc>** | Время выставления (UTC) | [default to null]
**rtype** | **OrderType** |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

