import { hexRequired } from '@/app/utils/form-validations'
import { useBattle } from '@/app/context'
import { useBattleMessage } from '@/app/hooks/use-battle'
import { useForm } from '@mantine/form'
import { ENV, createTamagotchiInitial } from '@/app/consts'
import { Button, Input } from '@gear-js/ui'
import { useHandleCalculateGas } from '@/app/hooks/use-handle-calculate-gas'
import { useCheckBalance } from '@/app/hooks/use-check-balance'
import { withoutCommas } from '@gear-js/react-hooks'

const validate: Record<string, typeof hexRequired> = {
  programId: hexRequired,
}

export const StartBattleForm = () => {
  const { battleState } = useBattle()
  const { sendMessage: handleMessage, metadata } = useBattleMessage()
  const calculateGas = useHandleCalculateGas(ENV.battle, metadata)
  const { checkBalance } = useCheckBalance()
  const form = useForm({
    initialValues: createTamagotchiInitial,
    validate,
    validateInputOnChange: true,
  })
  const { getInputProps, errors } = form
  const handleSubmit = form.onSubmit((values) => {
    const payload = { Register: { tmg_id: values.programId } }
    const onSuccess = () => form.reset()

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const minLimit = withoutCommas(min_limit as string)
        const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2)

        checkBalance(
          gasLimit,
          () => {
            handleMessage({
              payload,
              gasLimit,
              onSuccess,
            })
          },
          () => console.log('error')
        )
      })
      .catch((error) => {
        console.log(error)
      })
  })

  return (
    <div className="space-y-10 my-auto">
      <h2 className="text-center typo-h2">Registration for Battle</h2>
      <p className="text-center text-white text-opacity-70">
        Current players' queue: {battleState?.players.length ?? 0}
      </p>
      <form
        onSubmit={handleSubmit}
        className="flex items-start justify-center gap-6"
      >
        <div className="basis-[400px]">
          <Input
            placeholder="Insert program ID"
            direction="y"
            {...getInputProps('programId')}
          />
        </div>
        <div className="whitespace-nowrap">
          <Button
            text="Register Tamagotchi"
            color="primary"
            type="submit"
            disabled={Object.keys(errors).length > 0}
          />
        </div>
      </form>
    </div>
  )
}
