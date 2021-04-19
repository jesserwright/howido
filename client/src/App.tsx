import React, { Dispatch, useState } from 'react'
import { BrowserRouter, Route } from 'react-router-dom'
import './index.css'
import About from './pages/about'
import Home from './pages/home'
import Login from './pages/login'
import HowTo from './pages/howTo'
import { HelmetProvider } from 'react-helmet-async'
import Profile from './pages/profile'
import SignUp from './components/SignUp'
import Following from './pages/following'
import Followers from './pages/followers'
import AccountCreated from './pages/accountCreated'
import { SetStateAction } from 'react'

// What is the risk of not encoding/decoding? It is getting a bad response, and having the entire program panic :)
// I'm going to be OK with the type panic

// This needs to relate to the backend error type 1:1
type ServerError = {
  message: string
  fieldName: string
}

type GlobalError = ServerError | null

export const AppContext = React.createContext<{
  serverError: GlobalError
  setErrorMessage: Dispatch<SetStateAction<GlobalError>>
  loggedIn: boolean
  setLoggedIn: Dispatch<SetStateAction<boolean>>
}>({
  serverError: null,
  setErrorMessage: () => {},
  loggedIn: false,
  setLoggedIn: () => {},
})

function App() {
  const [loggedIn, setLoggedIn] = useState(false)
  const [errorMessage, setErrorMessage] = useState<GlobalError>(null)
  return (
    <AppContext.Provider
      value={{
        serverError: errorMessage,
        setErrorMessage,
        loggedIn,
        setLoggedIn,
      }}
    >
      <HelmetProvider>
        <BrowserRouter>
          <Route exact path="/">
            <Home />
          </Route>
          <Route path="/about">
            <About />
          </Route>
          <Route path="/login">
            <Login />
          </Route>
          <Route path="/profile">
            <Profile />
          </Route>
          <Route path="/how-to/:id">
            <HowTo />
          </Route>
          <Route path="/sign-up">
            <SignUp />
          </Route>
          <Route path="/followers">
            <Followers />
          </Route>
          <Route path="/following">
            <Following />
          </Route>
          <Route path="/account-created">
            <AccountCreated />
          </Route>
        </BrowserRouter>
      </HelmetProvider>
    </AppContext.Provider>
  )
}

export default App
