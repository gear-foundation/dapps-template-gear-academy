import type { ComponentType } from 'react'
import { BrowserRouter } from 'react-router-dom'
import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider,
  ProviderProps,
} from '@gear-js/react-hooks'
import { Alert, alertStyles } from '@gear-js/ui'
import { ENV } from '../consts'
import {
  AppProvider,
  BattleProvider,
  ItemsStoreProvider,
  LessonsProvider,
  TmgProvider,
  TokensBalanceProvider,
} from '../context'

const ApiProvider = ({ children }: ProviderProps) => (
  <GearApiProvider initialArgs={{ endpoint: ENV.NODE }}>
    {children}
  </GearApiProvider>
)

const AlertProvider = ({ children }: ProviderProps) => (
  <GearAlertProvider template={Alert} containerClassName={alertStyles.root}>
    {children}
  </GearAlertProvider>
)

const providers = [
  BrowserRouter,
  AlertProvider,
  ApiProvider,
  AccountProvider,
  AppProvider,
  LessonsProvider,
  TmgProvider,
  TokensBalanceProvider,
  ItemsStoreProvider,
  BattleProvider,
]

export const withProviders = (Component: ComponentType) => () =>
  providers.reduceRight(
    (children, Provider) => <Provider>{children}</Provider>,
    <Component />
  )
