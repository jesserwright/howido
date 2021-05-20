import { Link } from 'react-router-dom'
import { ChevronRight } from 'react-feather'
import React, { useState } from 'react'
import Layout from '../components/Layout'
import StyledLink from '../components/StyledLink'
import { wait150ms } from '../util/mockFunctions'

export default function Profile() {
  const [isFollowing, setFollowing] = useState(false)

  // use this if not logged in
  // const router = useRouter()

  // This should be an arg: `userToFollowId: number`

  async function followUser() {
    const _resp = await wait150ms('test')
    setFollowing(!isFollowing)
    // This is where following happens w/ the api.
    // If user is trying to make an action that requires being logged in, then send them to the login page:
    // `redirectToLogin()` (or `useRedirect(uri: string)` as a custom hook later)
    // If they are logged in, then the API can be called, with the user id to follow
    // `followUserRequest()`, THEN `setFollowing`
    // which will wait.... without a loading state for now
    // the API will be mocked for now, at a wait time of 100ms
    // ****This is all state managment being done on the client side(!) *****
    // So we have the user to follow id, and we need to get our current user id:
    // `getUserId()` (this gets the user id from the cookie or some other thing...)
  }

  const INSTRUCTIONS = [
    {
      id: 1,
      title: 'Morning Chicken Chores 🐓',
      seconds: 32,
      minutes: 8,
      steps: 8,
    },
    {
      id: 2,
      title: 'Chicken Soup',
      seconds: 3,
      minutes: 45,
      steps: 18,
    },
  ]

  const USER_NAME = 'Jesse Wright'
  return (
    <Layout pageTitle={USER_NAME}>
      <div className="flex flex-col md:flex-row items-stretch mx-auto">
        {/* profile area */}
        <div
          className="
            md:w-1/4
            md:pr-4
            md:border-r-2
            flex
            flex-col
            items-center
            mb-4 md:mb-0
          "
        >
          {/* IMAGE */}
          <img
            src={`${import.meta.env.API_URL}/images/733f55b5a10e4806968e207af934c9e2.jpg`}
            alt=""
            className="
              rounded-full
              shadow
              w-1/3 sm:w-1/4 md:w-full
              h-full
            "
          />

          <div
            className="
              flex flex-col md:block
              md:items-center
              justify-center
              w-full
            "
          >
            {/* NAME */}
            <h1
              className="
               mt-2
               text-2xl
               text-center
               font-bold
               break-words
             "
            >
              Jesse Wright
            </h1>

            {/* BIO */}
            <div
              className="
                  md:text-sm
                  text-center
                  mt-2 
                "
            >
              <span className="font-bold">“</span>
              Cycling, chickens, soup, and the simple things
              <span className="font-bold">”</span>
            </div>
            {/* END BIO */}

            {/* FOLLOW area */}
            <div
              className="
                    flex
                    flex-col
                    items-center
                    pt-2 md:pt-4
                    w-full
                    mt-1 md:mt-0
                  "
            >
              <div className="flex font-semibold mb-1">
                <StyledLink
                  href="/following"
                  title="following"
                  className="mr-2 md:mr-1"
                />
                <StyledLink href="/followers" title="followers" />
              </div>

              <button
                onClick={followUser}
                className={`
                  border py-1 md:py-0.5 px-4 rounded-full
                  transition-shadow
                  text-sm sm:text-base
                  shadow-sm hover:shadow
                  mt-2 md:mt-4 
                  w-full
                  ${!isFollowing && 'bg-white  font-medium'}
                `}
              >
                {isFollowing ? 'Unfollow' : 'Follow'}
              </button>
            </div>
          </div>
        </div>

        {/* INSTRUCTIONS area */}
        <div className="mt-4 md:mt-0 md:w-3/4 md:pl-4">
          <div className="flex flex-col">
            {INSTRUCTIONS.map(({ id, title, seconds, minutes, steps }) => {
              return (
                <Link
                  key={id}
                  to={`/how-to/${id - 1}`}
                  className="
                      group
                      w-full
                      shadow-sm
                      hover:shadow
                      transition-shadow
                      bg-white
                      flex
                      justify-between
                      items-center
                      border
                      rounded-md
                      py-2
                      px-3
                      md:py-3
                      md:px-4
                      mb-2
                    "
                >
                  <div
                    className="
                        md:flex
                        flex-grow
                        justify-between
                        items-center
                      "
                  >
                    <h2 className="text-lg font-semibold mb-1 md:mb-0">
                      {title}
                    </h2>

                    <span className="whitespace-no-wrap md:mx-4 text-sm">
                      ⏱ <strong>{minutes}</strong>m <strong>{seconds}</strong>
                      s&nbsp;&nbsp;
                      <span className="text-lg">|</span>&nbsp;&nbsp;
                      <strong>{steps}</strong> Steps
                    </span>
                  </div>
                  <ChevronRight className="group-hover:translate-x-0.5 transform transition-transform" />
                </Link>
              )
            })}
          </div>
        </div>
      </div>
    </Layout>
  )
}
