# Security

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**isin** | **String** | Идентификатор ценной бумаги согласно стандарту ISO 6166 | [optional] [default to null]
**cancellation** | **String** | Дата и время (UTC) окончания действия | [default to null]
**cfi_code** | **String** | Тип ценной бумаги согласно стандарту ISO 10962 | [default to null]
**complex_product_category** | **ComplexProductCategory** |  | [default to null]
**currency** | **String** | Валюта | [optional] [default to null]
**description** | **String** | Краткое описание инструмента | [default to null]
**exchange** | **Exchange** |  | [default to null]
**facevalue** | **Decimal** | Номинальная стоимость | [default to null]
**lotsize** | **Decimal** | Размер лота | [default to null]
**marginbuy** | **Decimal** | Цена маржинальной покупки (заемные средства) | [default to null]
**marginrate** | **Decimal** | Отношение цены маржинальной покупки к цене последней сделки | [default to null]
**marginsell** | **Decimal** | Цена маржинальной продажи (заемные средства) | [default to null]
**minstep** | **Decimal** | Минимальный шаг цены | [default to null]
**price_max** | **Decimal** | Максимальная цена | [default to null]
**price_min** | **Decimal** | Минимальная цена | [default to null]
**pricestep** | **Decimal** | Минимальный шаг цены, выраженный в рублях | [default to null]
**primary_board** | **String** | Код режима торгов | [default to null]
**rating** | **Decimal** |  | [default to null]
**shortname** | **String** | Краткое наименование инструмента | [default to null]
**symbol** | **String** | Тикер (Код финансового инструмента) | [default to null]
**theor_price** | **Decimal** |  | [default to null]
**theor_price_limit** | **Decimal** |  | [default to null]
**trading_status** | **i32** | Торговый статус инструмента:   * &#x60;18&#x60; - Нет торгов / торги закрыты   * &#x60;118&#x60; - Период открытия   * &#x60;103&#x60; - Период закрытия   * &#x60;2&#x60; - Перерыв в торгах   * &#x60;17&#x60; - Нормальный период торгов   * &#x60;102&#x60; - Аукцион закрытия   * &#x60;106&#x60; - Аукцион крупных пакетов   * &#x60;107&#x60; - Дискретный аукцион   * &#x60;119&#x60; - Аукцион открытия   * &#x60;120&#x60; - Период торгов по цене аукциона закрытия  | [default to null]
**trading_status_info** | **String** | Описание торгового статуса инструмента | [optional] [default to null]
**rtype** | **String** | Тип | [optional] [default to null]
**volatility** | **Decimal** | Волативность | [default to null]
**ryield** | **String** |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

