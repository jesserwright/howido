import 'tailwindcss/tailwind.css'
import '../styles/global.css'
import type { AppProps } from 'next/app'
import { useState } from 'react'
import React from 'react'
import { Context1 } from '../util/context'

function MyApp({ Component, pageProps }: AppProps) {
  // The logged in state needs to persist throught the *entire app*
  const [loggedIn, setLogin] = useState(false)
  // Onload, we'll need to check if the cookie exists? Or is that already sent?
  // I'd prefer for the client to check if the cookie is there, and change
  // the UI depending on that.

  return (
    <Context1.Provider value={{ loggedIn, setLogin }}>
      <Component {...pageProps} />
    </Context1.Provider>
  )
}

export default MyApp
