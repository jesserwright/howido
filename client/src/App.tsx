import React, { useState } from 'react'
import { BrowserRouter, Route } from 'react-router-dom'
import './index.css'
import { Context1 } from './util/context'
import About from './pages/about'
import Home from './pages/home'
import Login from './pages/login'
import HowTo from './pages/howTo'
import { HelmetProvider } from 'react-helmet-async'
import Profile from './pages/profile'
import { HOWTOS } from './util/STATIC_DB'
import SignUp from './pages/sign-up'
import Following from './pages/followers'
import Followers from './pages/followers'

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
          <Route path="/profile">
            <Profile />
          </Route>
          <Route path="/how-to">
            <HowTo howto={HOWTOS[0]} />
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
        </BrowserRouter>
      </HelmetProvider>
    </Context1.Provider>
  )
}

export default App
