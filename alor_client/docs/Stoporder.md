# Stoporder

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**broker_symbol** | **String** | Пара Биржа:Тикер | [default to null]
**end_time** | **DateTime<Utc>** | Время действия заявки (UTC) | [default to null]
**exchange** | **Exchange** |  | [default to null]
**existing** | **bool** | True - для данных из \&quot;снепшота\&quot;, то есть из истории. False - для новых событий | [default to null]
**filled_qty_batch** | [***Decimal**](BigDecimal.md) | Количество исполненных | [default to null]
**id** | [***Decimal**](BigDecimal.md) | Уникальный идентификатор стоп-заявки | [default to null]
**price** | [***Decimal**](BigDecimal.md) | Цена(Лимит) | [default to null]
**qty** | [***Decimal**](BigDecimal.md) | Количество | [default to null]
**side** | **Operation** |  | [default to null]
**status** | **OrderStatus** |  | [default to null]
**stop_price** | [***Decimal**](BigDecimal.md) | Условная цена | [default to null]
**symbol** | **String** | Тикер (Код финансового инструмента) | [default to null]
**rtype** | **StopOrderType** |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

