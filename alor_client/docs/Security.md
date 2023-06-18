# Security

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**isin** | **String** | Идентификатор ценной бумаги согласно стандарту ISO 6166 | [optional] [default to null]
**cancellation** | **String** | Дата и время (UTC) окончания действия | [default to null]
**cfi_code** | **String** | Тип ценной бумаги согласно стандарту ISO 10962 | [default to null]
**complex_product_category** | **String** | Требуемая категория для осуществления торговли инструментом | [optional] [default to null]
**currency** | **String** | Валюта | [default to null]
**description** | **String** | Краткое описание инструмента | [default to null]
**exchange** | **Exchange** |  | [default to null]
**facevalue** | [***Decimal**](BigDecimal.md) | Номинальная стоимость | [default to null]
**lotsize** | [***Decimal**](BigDecimal.md) | Размер лота | [default to null]
**marginbuy** | [***Decimal**](BigDecimal.md) | Цена маржинальной покупки (заемные средства) | [default to null]
**marginrate** | [***Decimal**](BigDecimal.md) | Отношение цены маржинальной покупки к цене последней сделки | [default to null]
**marginsell** | [***Decimal**](BigDecimal.md) | Цена маржинальной продажи (заемные средства) | [default to null]
**minstep** | [***Decimal**](BigDecimal.md) | Минимальный шаг цены | [default to null]
**price_max** | [***Decimal**](BigDecimal.md) | Максимальная цена | [default to null]
**price_min** | [***Decimal**](BigDecimal.md) | Минимальная цена | [default to null]
**pricestep** | [***Decimal**](BigDecimal.md) | Минимальный шаг цены, выраженный в рублях | [default to null]
**primary_board** | **String** | Код режима торгов | [default to null]
**rating** | [***Decimal**](BigDecimal.md) |  | [default to null]
**shortname** | **String** | Краткое наименование инструмента | [default to null]
**symbol** | **String** | Тикер (Код финансового инструмента) | [default to null]
**theor_price** | [***Decimal**](BigDecimal.md) |  | [default to null]
**theor_price_limit** | [***Decimal**](BigDecimal.md) |  | [default to null]
**trading_status** | **i32** | Торговый статус инструмента | [default to null]
**trading_status_info** | **String** | Описание торгового статуса инструмента | [optional] [default to null]
**rtype** | **String** | Тип | [default to null]
**volatility** | [***Decimal**](BigDecimal.md) | Волативность | [default to null]
**ryield** | **String** |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

