# Trade

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**board** | **String** | Код режима торгов (Борд) | [default to null]
**broker_symbol** | **String** | Пара Биржа:Тикер | [default to null]
**comment** | **String** | Пользовательский комментарий к заявке | [default to null]
**commission** | **Decimal** | Суммарная комиссия (null для Срочного рынка) | [optional] [default to null]
**date** | **DateTime<Utc>** | Дата и время (UTC) закрытия заявки | [default to null]
**exchange** | **Exchange** |  | [default to null]
**existing** | **bool** | &#x60;True&#x60; - для данных из \&quot;снепшота\&quot;, то есть из истории. &#x60;False&#x60; - для новых событий | [default to null]
**id** | **Decimal** | Уникальный идентификатор сделки | [default to null]
**orderno** | **String** | Идентификатор заявки | [default to null]
**price** | **Decimal** | Цена | [default to null]
**qty** | **i32** | Количество (лоты) | [default to null]
**qty_batch** | **i32** | Количество (лоты) | [default to null]
**qty_units** | **i32** | Количество (штуки) | [default to null]
**repo_specific_fields** | [***TradeRepoSpecificFields**](trade_repoSpecificFields.md) |  | [default to null]
**side** | **Side** |  | [default to null]
**symbol** | **String** | Тикер (Код финансового инструмента). \&quot;[N/A]\&quot; используется, если &#x60;symbol&#x60; не определен | [default to null]
**volume** | **Decimal** | Объём, рассчитанный по средней цене | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

