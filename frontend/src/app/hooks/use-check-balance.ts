import {
  useAccount,
  useAlert,
  useApi,
  useBalanceFormat,
  useVoucher,
  withoutCommas,
} from '@gear-js/react-hooks'
import { stringShorten } from '@polkadot/util'
import { useAccountAvailableBalance } from './use-account-available-balance'
import { ENV } from '../consts'

export function useCheckBalance(isVoucher?: boolean) {
  const { api } = useApi()
  const { account } = useAccount()
  const { availableBalance } = useAccountAvailableBalance()
  const { getChainBalanceValue } = useBalanceFormat()
  const { voucherBalance } = useVoucher(ENV.battle)
  const { getFormattedBalanceValue } = useBalanceFormat()
  const alert = useAlert()

  const checkBalance = (
    limit: number,
    callback: () => void,
    onError?: () => void
  ) => {
    const chainBalance = Number(
      getChainBalanceValue(
        Number(withoutCommas(availableBalance?.value || ''))
      ).toFixed()
    )
    const valuePerGas = Number(withoutCommas(api!.valuePerGas!.toHuman()))
    const chainEDeposit = Number(
      getChainBalanceValue(
        Number(withoutCommas(availableBalance?.existentialDeposit || ''))
      ).toFixed()
    )

    const chainEDepositWithLimit = chainEDeposit + limit * valuePerGas

    if (
      isVoucher && !!voucherBalance
        ? getFormattedBalanceValue(voucherBalance.toString()).toFixed() < 3
        : !chainBalance || chainBalance < chainEDepositWithLimit
    ) {
      alert.error(
        `Low balance on ${stringShorten(account?.decodedAddress || '', 8)}`
      )

      if (onError) {
        onError()
      }

      return
    }

    callback()
  }

  return { checkBalance }
}
