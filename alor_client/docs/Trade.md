# Trade

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**board** | **String** | Код режима торгов | [default to null]
**broker_symbol** | **String** | Пара Биржа:Тикер | [default to null]
**date** | **DateTime<Utc>** | Дата и время (UTC) закрытия заявки | [default to null]
**exchange** | **Exchange** |  | [default to null]
**existing** | **bool** | True - для данных из \&quot;снепшота\&quot;, то есть из истории. False - для новых событий | [default to null]
**id** | **Decimal** | Уникальный идентификатор сделки | [default to null]
**orderno** | **String** | Идентификатор заявки | [default to null]
**price** | **Decimal** | Цена | [default to null]
**qty** | **i32** | Количество (лоты) | [default to null]
**qty_batch** | **i32** | Количество (лоты) | [default to null]
**qty_units** | **i32** | Количество (штуки) | [default to null]
**side** | **Side** |  | [default to null]
**symbol** | **String** | Тикер (Код финансового инструмента). \&quot;[N/A]\&quot; используется если symbol не определен. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

