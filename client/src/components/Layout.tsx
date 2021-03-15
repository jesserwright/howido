import React from 'react'
import { Helmet } from 'react-helmet-async'
import { User } from 'react-feather'
import { Link } from 'react-router-dom'
import { AppContext } from '../App' // is this an instance of a dependency?

const Layout: React.FC<{ pageTitle: string; className?: string }> = (props) => {
  let { loggedIn } = React.useContext(AppContext)

  function handleLogout() {
    loggedIn = !loggedIn
  }

  return (
    <div
      className={`
        grid
        gap-y-4 md:gap-y-6
        grid-cols-1 md:grid-cols-custom
        grid-rows-custom
        min-h-screen
        bg-gray-100
        text-gray-900
        antialiased
    `}
    >
      <Helmet>
        <title>{props.pageTitle} | How I Do</title>
      </Helmet>
      <header
        className={`
          md:col-start-2
          col-start-1
          col-span-1
          row-start-1
          flex
          items-center
          relative
          z-20
       `}
      >
        <Link
          to={`/`}
          className="
              hover:text-gray-500
              active:text-gray-900
              transition-colors
              font-bold
              text-lg
              mx-auto
              flex
              items-center
              "
        >
          How I Do
        </Link>

        {loggedIn ? (
          // do something other than this plain image.. it should link to profile.
          <Link to={`/profile`} className="absolute right-4 w-8 ">
            <img
              className="shadow rounded-full"
              src={`https://github.com/identicons/jasonlong.png`}
              alt=""
            />
          </Link>
        ) : (
          <Link
            to={`/login`}
            className={`
              font-medium
              hover:text-gray-500
              transition-colors
              text-sm
              flex
              items-center
              absolute
              right-4
              `}
          >
            Login <User size={15} className="ml-1.5" />
          </Link>
        )}
      </header>

      {/* container just for bg color */}
      <div className="shadow-sm col-start-1 col-span-3 row-start-1 bg-white"></div>

      <main
        className={`
            px-4
            md:col-start-2
            col-span-1
            ${props.className}
        `}
      >
        {props.children}
      </main>

      <footer
        className="
          flex
          items-center
          justify-center
          col-start-1
          col-span-3
          border-t border-gray-200
          bg-gray-200
          text-gray-600
        "
      >
        <span className="text-sm">
          © <span className="font-bold">How I Do</span> since 2021&nbsp;&nbsp;
          <span className="text-sm">|</span>&nbsp;&nbsp;
          <Link to={`/about`} className="hover:underline">
            About
          </Link>
        </span>
      </footer>
    </div>
  )
}

export default Layout
