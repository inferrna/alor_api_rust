# StoporderWarp

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**avg_price** | **Decimal** | Средняя цена | [default to null]
**broker_symbol** | **String** | Пара Биржа:Тикер | [default to null]
**condition** | **Condition** |  | [default to null]
**end_time** | **DateTime<Utc>** | Дата и время завершения (UTC) | [default to null]
**exchange** | **Exchange** |  | [default to null]
**exchange_order_id** | **i64** | Уникальный идентификатор биржевой заявки | [default to null]
**existing** | **bool** | &#x60;True&#x60; - для данных из \&quot;снепшота\&quot;, то есть из истории. &#x60;False&#x60; - для новых событий | [default to null]
**filled** | **Decimal** | Количество исполненных (штуки) | [default to null]
**filled_qty_batch** | **Decimal** | Количество исполненных (лоты) | [default to null]
**filled_qty_units** | **Decimal** | Количество исполненных (штуки) | [default to null]
**id** | **Decimal** | Уникальный идентификатор стоп-заявки | [default to null]
**portfolio** | **String** | Идентификатор клиентского портфеля | [default to null]
**price** | **Decimal** | Цена (Лимит) | [default to null]
**qty** | **i32** | Количество (лоты) | [default to null]
**qty_batch** | **i32** | Количество (лоты) | [default to null]
**qty_units** | **Decimal** | Количество (штуки) | [default to null]
**side** | **Side** |  | [default to null]
**status** | **OrderStatus** |  | [default to null]
**stop_price** | **Decimal** | Условная цена | [default to null]
**symbol** | **String** | Тикер (Код финансового инструмента) | [default to null]
**trans_time** | **DateTime<Utc>** | Дата и время выставления (UTC) | [default to null]
**rtype** | **StopOrderType** |  | [default to null]
**volume** | **Decimal** | Объем, для рыночных заявок - null | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

