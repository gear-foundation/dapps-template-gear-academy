import { HexString, ProgramMetadata } from '@gear-js/api'
import {
  useAlert,
  withoutCommas,
  useHandleCalculateGas as useCalculateGasNative,
} from '@gear-js/react-hooks'
import { AnyJson, AnyNumber } from '@polkadot/types/types'
import { useAccountAvailableBalance } from './use-account-available-balance'

export const useHandleCalculateGas = (
  address: HexString,
  meta?: ProgramMetadata
) => {
  const { availableBalance } = useAccountAvailableBalance()
  const calculateGasNative = useCalculateGasNative(address, meta)

  const alert = useAlert()

  return (initPayload: AnyJson, value?: AnyNumber | undefined) => {
    const balance = Number(withoutCommas(availableBalance?.value || ''))
    const existentialDeposit = Number(
      withoutCommas(availableBalance?.existentialDeposit || '')
    )

    if (!balance || balance < existentialDeposit) {
      alert.error(`Low balance when calculating gas`)
    }

    return calculateGasNative(initPayload, value)
  }
}
