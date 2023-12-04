import { Button, Input } from '@gear-js/ui'
import { useForm } from '@mantine/form'
import { hexRequired } from '@/app/utils/form-validations'
import { useApp } from '@/app/context'
import { useTamagotchiMessage } from '@/app/hooks/use-tamagotchi'
import { SpriteIcon } from '@/components/ui/sprite-icon'
import { ENV } from '@/app/consts'
import { useHandleCalculateGas } from '@/app/hooks/use-handle-calculate-gas'
import { withoutCommas } from '@gear-js/react-hooks'
import { useCheckBalance } from '@/app/hooks/use-check-balance'

const initialValues = {
  address: '',
}

const validate = {
  address: hexRequired,
}

export const TransferAccountForm = ({ close }: { close: () => void }) => {
  const { isPending } = useApp()
  const { send: sendHandler, lessonMeta } = useTamagotchiMessage()
  const calculateGas = useHandleCalculateGas(ENV.battle, lessonMeta)
  const { checkBalance } = useCheckBalance()
  const form = useForm({ initialValues, validate })
  const { getInputProps } = form
  const onSuccess = () => close()

  const handleSubmit = form.onSubmit((values) => {
    const payload = { Transfer: values.address }

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const minLimit = withoutCommas(min_limit as string)
        const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2)

        checkBalance(
          gasLimit,
          () => {
            sendHandler({
              payload,
              gasLimit,
              onSuccess,
            })
          },
          () => {
            console.log('error')
          }
        )
      })
      .catch((error) => {
        console.log(error)
      })
  })

  return (
    <form className="space-y-6" onSubmit={handleSubmit}>
      <div className="">
        <Input
          placeholder="Enter the account address"
          {...getInputProps('address')}
        />
      </div>
      <div className="whitespace-nowrap">
        <Button
          text="Send"
          color="primary"
          type="submit"
          icon={() => <SpriteIcon name="transfer" className="w-5 h-5" />}
          className="w-full gap-2"
          disabled={isPending}
        />
      </div>
    </form>
  )
}
