import './global.css'
import './app.scss'
import { useApi, useAccount } from '@gear-js/react-hooks'
import { Routing } from './pages'
import { ApiLoader } from './components/loaders/api-loader'
import { Footer, Header } from '@/components/layout'
import { withProviders } from '@/app/hocs'
import { useAccountAvailableBalanceSync } from './app/hooks/use-account-available-balance'
import { useEffect } from 'react'
import { ENV } from './app/consts'

const Component = () => {
  const { isApiReady } = useApi()
  const { isAccountReady } = useAccount()

  useAccountAvailableBalanceSync()

  useEffect(() => {
    console.log('Node')
    console.log(ENV.NODE)
    console.log('store')
    console.log(ENV.store)
    console.log('battle')
    console.log(ENV.battle)
  }, [])

  return (
    <div className="flex flex-col min-h-screen">
      <Header />
      <main className="flex flex-col flex-1 container pt-3 pb-5">
        {isApiReady && isAccountReady ? <Routing /> : <ApiLoader />}
      </main>
      <Footer />
    </div>
  )
}

export const App = withProviders(Component)
