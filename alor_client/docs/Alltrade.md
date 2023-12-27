# Alltrade

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**existing** | **bool** | &#x60;True&#x60; - для данных из \&quot;снепшота\&quot;, то есть из истории. &#x60;False&#x60; - для новых событий | [default to null]
**id** | **i64** | Уникальный идентификатор | [default to null]
**oi** | **i32** | Открытый интерес (open interest). Если не поддерживается инструментом — значение 0 | [default to null]
**orderno** | **i64** | Идентификатор заявки | [default to null]
**price** | **Decimal** | Цена | [default to null]
**qty** | **i32** | Количество | [default to null]
**side** | **Side** |  | [default to null]
**symbol** | **String** | Тикер (Код финансового инструмента). \&quot;[N/A]\&quot; используется, если &#x60;symbol&#x60; не определен | [default to null]
**time** | **DateTime<Utc>** | Дата и время (UTC) закрытия заявки | [default to null]
**timestamp** | **i64** | Время (UTC) в формате Unix Time Milliseconds | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

