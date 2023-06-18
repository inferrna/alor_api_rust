# Fortsrisk

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**balance_money** | [***Decimal**](BigDecimal.md) | Сальдо денежных торговых переводов за текущую сессию (поле будет удалено в будущих обновлениях) | [default to null]
**fee** | [***Decimal**](BigDecimal.md) | Списанный сбор | [default to null]
**is_limits_set** | **bool** | Наличие установленных денежного и залогового лимитов | [default to null]
**money_amount** | [***Decimal**](BigDecimal.md) | Общее количество рублей и дисконтированных в рубли залогов | [default to null]
**money_blocked** | [***Decimal**](BigDecimal.md) | Средства, заблокированные под ГО | [default to null]
**money_free** | [***Decimal**](BigDecimal.md) | Свободные средства. Сумма рублей и залогов, дисконтированных в рубли, доступная для открытия позиций. (MoneyFree &#x3D; MoneyAmount + VmInterCl – MoneyBlocked – VmReserve – Fee) | [default to null]
**money_old** | [***Decimal**](BigDecimal.md) | Общее количество рублей и дисконтированных в рубли залогов на начало сессии | [default to null]
**money_pledge_amount** | [***Decimal**](BigDecimal.md) | Сумма залогов, дисконтированных в рубли | [default to null]
**portfolio** | **String** | Идентификатор клиентского портфеля | [default to null]
**var_margin** | [***Decimal**](BigDecimal.md) | VmCurrentPositions + VmInterCl | [default to null]
**vm_current_positions** | [***Decimal**](BigDecimal.md) | Сагрегированная вармаржа по текущим позициям | [default to null]
**vm_inter_cl** | [***Decimal**](BigDecimal.md) | Вариационная маржа, списанная или полученная в пром. клиринг | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

