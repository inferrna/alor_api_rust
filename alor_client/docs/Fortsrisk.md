# Fortsrisk

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**balance_money** | **Decimal** | Сальдо денежных торговых переводов за текущую сессию (поле будет удалено в будущих обновлениях) | [default to null]
**fee** | **Decimal** | Списанный сбор | [default to null]
**is_limits_set** | **bool** | Наличие установленных денежного и залогового лимитов | [default to null]
**money_amount** | **Decimal** | Общее количество рублей и дисконтированных в рубли залогов | [default to null]
**money_blocked** | **Decimal** | Средства, заблокированные под ГО | [default to null]
**money_free** | **Decimal** | Свободные средства. Сумма рублей и залогов, дисконтированных в рубли, доступная для открытия позиций. (&#x60;MoneyFree&#x60; &#x3D; &#x60;MoneyAmount&#x60; + &#x60;VmInterCl&#x60; – &#x60;MoneyBlocked&#x60; – &#x60;VmReserve&#x60; – &#x60;Fee&#x60;) | [default to null]
**money_old** | **Decimal** | Общее количество рублей и дисконтированных в рубли залогов на начало сессии | [default to null]
**money_pledge_amount** | **Decimal** | Сумма залогов, дисконтированных в рубли | [default to null]
**portfolio** | **String** | Идентификатор клиентского портфеля | [default to null]
**var_margin** | **Decimal** | &#x60;VmCurrentPositions&#x60; + &#x60;VmInterCl&#x60; | [default to null]
**vm_current_positions** | **Decimal** | Сагрегированная вармаржа по текущим позициям | [default to null]
**vm_inter_cl** | **Decimal** | Вариационная маржа, списанная или полученная в пром. клиринг | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

