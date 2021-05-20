import React, { Dispatch, useState } from 'react'
import { BrowserRouter, Route } from 'react-router-dom'
import './index.css'
import About from './pages/About'
import Home from './pages/Home'
import { HelmetProvider } from 'react-helmet-async'
import SignUp from './components/SignUp'
import Following from './pages/Following'
import Followers from './pages/Followers'
import AccountCreated from './pages/AccountCreated'
import { SetStateAction } from 'react'
import HowTo from './pages/HowTo'
import Login from './pages/Login'
import Profile from './pages/Profile'
import { SWRConfig } from 'swr'
import ImageUpload from './pages/ImageUpload'

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

const fetcher = async (
  input: RequestInfo,
  init: RequestInit,
  ...args: any[]
) => {
  const res = await fetch(import.meta.env.API_URL + input, init)
  if (!res.ok) {
    const error = new Error('An error occurred while fetching the data.')
    throw error
  }
  return res.json()
}

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
      <SWRConfig value={{ fetcher }}>
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
            <Route path="/image-upload">
              <ImageUpload />
            </Route>
          </BrowserRouter>
        </HelmetProvider>
      </SWRConfig>
    </AppContext.Provider>
  )
}

export default App
