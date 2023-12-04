import { useAccount, withoutCommas } from '@gear-js/react-hooks'
import { useApp, useLessons, useTamagotchi } from '@/app/context'
import { useLesson5 } from '@/app/hooks/use-lesson-5'
import { useTamagotchiMessage } from '@/app/hooks/use-tamagotchi'
import { NotificationResponseTypes } from '@/app/types/lessons'
import metaBattle from '@/assets/meta/meta-battle.txt'
import { cn, getNotificationTypeValue } from '@/app/utils'
import { AccountActionsMenu } from '@/components/menus/account-actions-menu'
import { getTamagotchiAge } from '@/app/utils/get-tamagotchi-age'
import { TamagotchiInfoCardRow } from '@/components/tamagotchi/tamagotchi-info-card-row'
import { useHandleCalculateGas } from '@/app/hooks/use-handle-calculate-gas'
import { ENV } from '@/app/consts'
import { useCheckBalance } from '@/app/hooks/use-check-balance'

export const TamagotchiInfoCard = () => {
  const { account } = useAccount()
  const { setIsPending } = useApp()
  const { lesson } = useLessons()
  const { tamagotchi } = useTamagotchi()
  const { setNotification, activeNotification, setActiveNotification } =
    useLesson5()
  const { checkBalance } = useCheckBalance()
  const { send, lessonMeta } = useTamagotchiMessage()
  const calculateGas = useHandleCalculateGas(ENV.battle, lessonMeta)

  const fullView = Boolean(lesson && lesson?.step > 1)

  const onSuccess = (str: NotificationResponseTypes) => {
    if (activeNotification) {
      setNotification((prev) => ({
        ...prev,
        ...getNotificationTypeValue(str),
      }))
      setActiveNotification(undefined)
    }
    setIsPending(false)
  }
  const onError = () => setIsPending(false)
  const feedHandler = () => {
    const payload = { Feed: null }

    setIsPending(true)

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const minLimit = withoutCommas(min_limit as string)
        const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2)

        checkBalance(
          gasLimit,
          () => {
            send({
              payload,
              gasLimit,
              onSuccess: () => onSuccess('FeedMe'),
              onError,
            })
          },
          onError
        )
      })
      .catch((error) => {
        console.log(error)
        onError()
      })
  }
  const playHandler = () => {
    const payload = { Play: null }

    setIsPending(true)

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const minLimit = withoutCommas(min_limit as string)
        const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2)

        checkBalance(
          gasLimit,
          () => {
            send({
              payload,
              gasLimit,
              onSuccess: () => onSuccess('PlayWithMe'),
              onError,
            })
          },
          onError
        )
      })
      .catch((error) => {
        console.log(error)
        onError()
      })
  }
  const sleepHandler = () => {
    const payload = { Sleep: null }

    setIsPending(true)

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const minLimit = withoutCommas(min_limit as string)
        const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2)

        checkBalance(
          gasLimit,
          () => {
            send({
              payload,
              gasLimit,
              onSuccess: () => onSuccess('WantToSleep'),
              onError,
            })
          },
          onError
        )
      })
      .catch((error) => {
        console.log(error)
        onError()
      })
  }

  return (
    <>
      {tamagotchi && (
        <div
          className={cn(
            'flex gap-12 items-center p-4 bg-white/5 rounded-2xl',
            fullView && 'w-full pr-12'
          )}
        >
          <div className="basis-[415px] w-full px-8 py-6 bg-[#1E1E1E] rounded-2xl">
            <div className="flex justify-between gap-4">
              <h2 className="typo-h2 text-primary truncate max-w-[9ch]">
                {tamagotchi.name ? tamagotchi.name : 'Geary'}
              </h2>
              <div>
                <AccountActionsMenu />
              </div>
            </div>
            <div className="mt-8 text-white text-lg font-medium">
              <table className="block w-full text-left">
                <tbody className="block space-y-8">
                  {tamagotchi.owner && (
                    <tr className="flex gap-8">
                      <th className="flex-1 w-40 text-white text-opacity-70 font-medium">
                        Owner ID:
                      </th>
                      <td className="flex-1 w-40 truncate">
                        {tamagotchi.owner === account?.decodedAddress
                          ? account?.meta.name
                          : tamagotchi.owner}
                      </td>
                    </tr>
                  )}
                  {tamagotchi.dateOfBirth && (
                    <tr className="flex gap-8">
                      <th className="flex-1 w-40 text-white text-opacity-70 font-medium">
                        Age:
                      </th>
                      <td className="flex-1 w-40">
                        {getTamagotchiAge(tamagotchi.dateOfBirth)}
                      </td>
                    </tr>
                  )}
                </tbody>
              </table>
            </div>
          </div>
          {fullView && (
            <div className="basis-[620px] w-full space-y-6 grow">
              <TamagotchiInfoCardRow
                label="Hungry"
                value={+withoutCommas(tamagotchi.fed)}
                icon="feed"
                labelBtn="Feed"
                onClick={feedHandler}
                tooltipText='Your character has a low fed score. In order to increase the level, please click on the "Feed" button'
                tooltipTitle="Low level of fed"
                isActive={activeNotification === 'FeedMe'}
              />
              <TamagotchiInfoCardRow
                label="Happy"
                value={+withoutCommas(tamagotchi.entertained)}
                icon="happy"
                labelBtn="Play"
                onClick={playHandler}
                tooltipText='Your character has a low happiness score. In order to increase the level, please click on the "Play" button'
                tooltipTitle="Low level of happiness"
                isActive={activeNotification === 'PlayWithMe'}
              />
              <TamagotchiInfoCardRow
                label="Tired"
                value={+withoutCommas(tamagotchi.rested)}
                icon="tired"
                labelBtn="Sleep"
                onClick={sleepHandler}
                tooltipText='Your character has a low rest score. In order to increase the level, please click on the "Sleep" button'
                tooltipTitle="Low level of rest"
                isActive={activeNotification === 'WantToSleep'}
              />
            </div>
          )}
        </div>
      )}
    </>
  )
}
