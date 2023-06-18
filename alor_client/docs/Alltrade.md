# Alltrade

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**existing** | **bool** | True - для данных из \&quot;снепшота\&quot;, то есть из истории. False - для новых событий | [default to null]
**id** | **i32** | Уникальный идентификатор. | [default to null]
**oi** | **i32** | Открытый интерес (open interest). Если не поддерживается инстурментом - значение 0. | [default to null]
**orderno** | **i32** | Идентификатор заявки | [default to null]
**price** | [***Decimal**](BigDecimal.md) | Цена | [default to null]
**qty** | **i32** | Количество | [default to null]
**side** | **Operation** |  | [default to null]
**symbol** | **String** | Тикер (Код финансового инструмента). \&quot;[N/A]\&quot; используется если symbol не определен. | [default to null]
**time** | **String** | Дата и время (UTC) закрытия заявки | [default to null]
**timestamp** | **i32** | Время (UTC) в формате Unix Time Milliseconds | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

