# Orderbook

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asks** | [**Vec<OrderbookAsk>**](orderbook_ask.md) | Аски | [default to null]
**bids** | [**Vec<OrderbookBid>**](orderbook_bid.md) | Биды | [default to null]
**existing** | **bool** | True - для данных из \&quot;снепшота\&quot;, то есть из истории. False - для новых событий | [default to null]
**ms_timestamp** | **i64** | Время(UTC) в формате Unix Time Milliseconds | [default to null]
**snapshot** | **bool** | Deprecated. Устаревшее поле, будет удалено в будущих обновлениях. | [default to null]
**timestamp** | [***Decimal**](BigDecimal.md) | Deprecated. Устаревшее поле, будет удалено в будущих обновлениях. Вместо этого поля используйте поле \&quot;ms_timestamp\&quot;. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

