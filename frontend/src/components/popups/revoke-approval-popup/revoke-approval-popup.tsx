import { Button, Modal } from '@gear-js/ui'
import { useTamagotchiMessage } from '@/app/hooks/use-tamagotchi'
import { useApp } from '@/app/context'
import { ENV } from '@/app/consts'
import { useCheckBalance } from '@/app/hooks/use-check-balance'
import { useHandleCalculateGas } from '@/app/hooks/use-handle-calculate-gas'
import { withoutCommas } from '@gear-js/react-hooks'

export const RevokeApprovalPopup = ({ close }: { close: () => void }) => {
  const { isPending } = useApp()
  const { checkBalance } = useCheckBalance()
  const { send: sendHandler, lessonMeta } = useTamagotchiMessage()
  const calculateGas = useHandleCalculateGas(ENV.battle, lessonMeta)

  const onSuccess = () => close()

  const handler = () => {
    const payload = { RevokeApproval: null }

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
  }

  return (
    <Modal heading="Revoke approval" close={close}>
      <div className="flex gap-6">
        <Button
          text="Dismiss"
          color="secondary"
          onClick={close}
          disabled={isPending}
        />
        <Button
          text="Ok"
          color="primary"
          onClick={handler}
          disabled={isPending}
        />
      </div>
    </Modal>
  )
}
