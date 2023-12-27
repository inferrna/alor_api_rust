# Orderbook

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asks** | [**Vec<OrderbookBid>**](orderbook_bid.md) | Аски | [default to null]
**bids** | [**Vec<OrderbookBid>**](orderbook_bid.md) | Биды | [default to null]
**existing** | **bool** | &#x60;True&#x60; - для данных из \&quot;снепшота\&quot;, то есть из истории. &#x60;False&#x60; - для новых событий | [default to null]
**ms_timestamp** | **i64** | Время (UTC) в формате Unix Time Milliseconds | [default to null]
**snapshot** | **bool** | Deprecated. Устаревшее поле, будет удалено в будущих обновлениях | [default to null]
**timestamp** | **Decimal** | Deprecated. Устаревшее поле, будет удалено в будущих обновлениях. Вместо этого поля используйте поле &#x60;ms_timestamp&#x60; | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

