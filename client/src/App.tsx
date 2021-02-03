import React, { useState } from 'react'
import { BrowserRouter, Route } from 'react-router-dom'
import './index.css'
import { Context1 } from './util/context'
import About from './pages/about'
import Home from './pages/home'
import Login from './pages/login'
import { HelmetProvider } from 'react-helmet-async'

function App() {
  // The logged in state needs to persist throught the *entire app*
  const [loggedIn, setLogin] = useState(false)
  // Onload, we'll need to check if the cookie exists? Or is that already sent?
  // I'd prefer for the client to check if the cookie is there, and change
  // the UI depending on that.

  return (
    <Context1.Provider value={{ loggedIn, setLogin }}>
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
        </BrowserRouter>
      </HelmetProvider>
    </Context1.Provider>
  )
}

export default App
