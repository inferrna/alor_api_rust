# RiskRate

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_type** | **String** | Тип актива | [default to null]
**created_at** | **String** | Время добавления ставки риска | [default to null]
**currency_code** | **String** | Код валюты расчетов | [default to null]
**exchange** | **Exchange** |  | [default to null]
**id** | [***Decimal**](BigDecimal.md) | Id записи | [default to null]
**instrument** | **String** | Инструмент | [default to null]
**is_direct** | **bool** | Является ли зависимость инструмента к базовому активу прямой или обратной. | [default to null]
**is_marginal** | **bool** | Доступен ли данный инструмент в маржу. Т.е. есть ли он в списке маржинальных инструментов брокера. | [default to null]
**is_short_sell_possible** | **bool** | Разрешен ли шорт по бумаге. True если да. | [default to null]
**isin** | **String** | ISIN инструмента. Если есть. | [default to null]
**rate_down** | [***Decimal**](BigDecimal.md) | Ставка риска понижения цены. Применяется для лонгов. | [default to null]
**rate_symmetric** | [***Decimal**](BigDecimal.md) | Симметричная ставка риска. Приведена для справки, не используется | [default to null]
**rate_up** | [***Decimal**](BigDecimal.md) | Ставка риска повышения цены. Применяется для шортов. | [default to null]
**risk_category_id** | [***Decimal**](BigDecimal.md) | Id категории риска | [default to null]
**security_risk_category_id** | [***Decimal**](BigDecimal.md) | Id категории бумаги для категоризации.  | [optional] [default to null]
**set_name** | **String** | Чаще всего будет null. Поле показывает к множеству инструменту принадлежит данный инструмент. | [optional] [default to null]
**set_rate** | [***Decimal**](BigDecimal.md) | Ставка риска множества | [default to null]
**underlying_asset** | **String** | Чаще всего будет null. Поле показывает к какому базовому инструменту принадлежит данный инструмент. | [optional] [default to null]
**updated_at** | **String** | Время последнего обновления ставки риска | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

