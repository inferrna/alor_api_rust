# StoporderWarp

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**avg_price** | [***Decimal**](BigDecimal.md) | Средняя цена | [default to null]
**broker_symbol** | **String** | Пара Биржа:Тикер | [default to null]
**condition** | **String** | Условие срабатывания more/less | [default to null]
**end_time** | **DateTime<Utc>** | Время действия заявки (UTC) | [default to null]
**exchange** | **Exchange** |  | [default to null]
**exchange_order_id** | [***Decimal**](BigDecimal.md) | Уникальный идентификатор стоп-заявки | [default to null]
**existing** | **bool** | True - для данных из \&quot;снепшота\&quot;, то есть из истории. False - для новых событий | [default to null]
**id** | [***Decimal**](BigDecimal.md) | Уникальный идентификатор стоп-заявки | [default to null]
**portfolio** | **String** | Пара Биржа:Тикер | [default to null]
**price** | [***Decimal**](BigDecimal.md) | Цена(Лимит) | [default to null]
**qty** | [***Decimal**](BigDecimal.md) | Количество (Лоты) | [default to null]
**qty_batch** | [***Decimal**](BigDecimal.md) | Количество (Лоты) | [default to null]
**qty_units** | [***Decimal**](BigDecimal.md) | Количество (Штуки) | [default to null]
**side** | **Operation** |  | [default to null]
**status** | **OrderStatus** |  | [default to null]
**stop_price** | [***Decimal**](BigDecimal.md) | Условная цена | [default to null]
**symbol** | **String** | Тикер (Код финансового инструмента) | [default to null]
**trans_time** | **DateTime<Utc>** | Время выставления заявки (UTC) | [default to null]
**rtype** | **StopOrderType** |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

