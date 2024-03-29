import { useEffect } from 'react'
import { useReadWasmState, withoutCommas } from '@gear-js/react-hooks'
import { useLessons, useTamagotchi } from '@/app/context'
import { useProgramMetadata, useStateMetadata } from './use-metadata'
import { sleep } from '@/app/utils'
import type { TamagotchiState } from '@/app/types/lessons'
import state2 from '@/assets/meta/state2.meta.wasm?url'

type StateWasmResponse = {
  fed: string
  entertained: string
  rested: string
}

export function useThrottleWasmState() {
  const { lesson, setIsReady, isReady } = useLessons()
  const metadata = useStateMetadata(state2)
  const programMeta = useProgramMetadata(state2)
  const { tamagotchi, setTamagotchi } = useTamagotchi()

  const { state } = useReadWasmState<StateWasmResponse>({
    programId: lesson?.programId,
    wasm: metadata?.buffer,
    programMetadata: programMeta,
    functionName: 'current_state',
  })

  useEffect(() => {
    if (lesson && lesson.step < 2) return
    if (state) {
      const { fed, rested, entertained } = state

      setTamagotchi({
        ...tamagotchi,
        ...state,
        isDead:
          [
            +withoutCommas(fed),
            +withoutCommas(rested),
            +withoutCommas(entertained),
          ].reduce((sum, a) => sum + +a) === 0,
      } as TamagotchiState)

      sleep(1).then(() => {
        if (lesson && lesson.step > 1) {
          !isReady && setIsReady(true)
        }
      })
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [state, lesson, isReady])
}
