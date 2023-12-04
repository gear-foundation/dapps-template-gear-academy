import { Account } from '@gear-js/react-hooks'
import { SpriteIcon } from '@/components/ui/sprite-icon'

type Props = {
  balance: Account['balance']
  address: string
  name: string | undefined
  onClick: () => void
}

export function GasWallet({ balance, address, name, onClick }: Props) {
  return (
    <div className="flex gap-4 shrink-0 items-center">
      <SpriteIcon
        name={
          balance?.unit?.toLowerCase() === 'vara' ? 'vara-coin' : 'tvara-coin'
        }
        width={28}
        height={28}
      />
      <p className="shrink-0 grid grid-cols-[auto_auto] gap-x-1 font-kanit">
        <span className="font-medium text-lg leading-none">
          {balance?.value}
        </span>
        <span className="text-sm text-white text-opacity-70">
          {balance?.unit}
        </span>
      </p>
    </div>
  )
}
