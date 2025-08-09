import { Provider } from './components/ui/provider'
import { ApiProvider } from '@/context/ApiProvider'
import { BrowserRouter } from 'react-router'
import { Router } from './Router'

function App() {
  return (
    <Provider>
      <BrowserRouter>
        <ApiProvider>
          <Router />
        </ApiProvider>
      </BrowserRouter>
    </Provider>
  )
}

export default App
