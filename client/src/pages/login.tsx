import React from 'react'
import Layout from '../components/Layout'
import StyledLink from '../components/StyledLink'
import { ChevronRight } from 'react-feather'
import { Context1, Context1Type } from '../util/context'

export default function Login() {
  // Multi-stage. One thing at a time
  // 1. Input email. There's an 'enter' button.
  // 2. Email exists. Email input is now non-interactive. A green checkmark appears next to it.
  // 3. Password field appears, along with login button. "Login with link instead" button also is there

  // SUCCESS: redirect to the profile page
  // FAIL: error text on the field
  // WAIT: a loading spinner appears

  // if (email === 'jessewright01@gmail.com') {
  // }
  // if (password === 'e$&auzX&') {
  // }

  // The local state needs to be 'logged in'

  // If already logged in, this route should redirect to the profile page.

  const ctx = React.useContext(Context1) as Context1Type

  function handleLogin() {
    ctx.setLogin(!ctx.loggedIn)
  }

  if (ctx.loggedIn) {
    // Redirect to the profile page or show "You're logged in"
  }

  return (
    <Layout pageTitle="Login" className="flex flex-col w-72 md:w-80 mx-auto">
      <h2 className="text-4xl font-bold mb-6 mt-4 text-center">Log In</h2>
      <label className="block mb-4">
        <span className="">Email Address</span>
        {/* TODO: autofocus */}
        <input
          type="email"
          autoFocus
          className="
            mt-1
            block
            w-full
            rounded-md
            border-transparent
            focus:border-gray-500
            focus:ring-0
            "
          placeholder=""
        />
      </label>

      <label className="block">
        <span className="">Password</span>
        <input
          type="password"
          className="
            mt-1
            block
            w-full
            rounded-md
            border-transparent
            focus:border-gray-500
            focus:ring-0
            "
          placeholder=""
        />
      </label>

      <button
        onClick={handleLogin}
        className="

          text-center
          w-full
          rounded-md
          border-gray-900
          border-2
          hover:bg-gray-900
          text-gray-900
          hover:text-white
          mt-5
          py-1
          transition-colors
          delay-75
          flex
          justify-center
          items-center
      "
      >
        Log In <ChevronRight className="ml-1" size={20} />
      </button>

      <hr className="w-full my-5 md:my-8" />

      <p className="mb-3 mx-auto">No account yet?</p>
      <StyledLink
        title="Sign Up"
        href={`/sign-up`}
        className="text-base font-semibold mx-auto"
      />
    </Layout>
  )
}
