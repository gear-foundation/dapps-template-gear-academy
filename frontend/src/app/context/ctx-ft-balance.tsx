import type { ProviderProps } from '@gear-js/react-hooks'
import { createContext } from 'react'
import { ProgramMetadata } from '@gear-js/api'
import meta from '@/assets/meta/sharded_fungible_token_meta.txt'
import metaLogic from '@/assets/meta/sharded_fungible_token_logic_meta.txt'
import metaStorage from '@/assets/meta/sharded_fungible_token_storage_meta.txt'
import { HexString } from '@polkadot/util/types'
import { ENV } from '@/app/consts'
import { useProgramMetadata } from '@/app/hooks/use-metadata'

function useProgram() {
  const metadata = useProgramMetadata(meta)
  const metaL = useProgramMetadata(metaLogic)
  const metaS = useProgramMetadata(metaStorage)
  return {
    programId: ENV.balance as HexString,
    metaMain: metadata as ProgramMetadata,
    metaLogic: metaL as ProgramMetadata,
    metaStorage: metaS as ProgramMetadata,
  }
}

type Program = ReturnType<typeof useProgram>

export const FTBalanceCtx = createContext<Program>(
  {} as ReturnType<typeof useProgram>
)

export function TokensBalanceProvider({ children }: ProviderProps) {
  const { Provider } = FTBalanceCtx
  return <Provider value={useProgram()}>{children}</Provider>
}
