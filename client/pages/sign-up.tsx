import Layout from '../components/Layout'
import Link from 'next/link'
import { Dispatch, SetStateAction, useState } from 'react'
import { ExternalLink } from 'react-feather'

const SignUpFields: React.FC<{
  setAccountCreated: Dispatch<SetStateAction<boolean>>
}> = (props) => {
  function handleSetAccountCreated() {
    props.setAccountCreated(true)
  }

  return (
    <>
      <label className="block mb-2">
        {/* TODO: autofocus */}
        <input
          type="text"
          className="
            w-full
            mt-1
            block
            rounded-md
            border-transparent
            focus:border-gray-500
            focus:ring-0
            "
          placeholder="Full Name"
        />
      </label>

      <label className="block mb-2">
        {/* TODO: auto-lowercase the email */}
        <input
          type="email"
          className="
            w-full
            mt-1
            block
            rounded-md
            border-transparent
            focus:border-gray-500
            focus:ring-0
            "
          placeholder="Email"
        />
      </label>

      <button
        onClick={handleSetAccountCreated}
        className="
            text-center
            w-full
            rounded-md
            border-gray-900
            border-2
            hover:bg-gray-900
            text-gray-900
            hover:text-white
            my-6
            py-1
            transition-colors
            delay-75
        "
      >
        Sign Up
      </button>
    </>
  )
}

export default function SignUp() {
  const [accountCreated, setAccountCreated] = useState(false)

  return (
    <Layout pageTitle="Sign Up" className="flex flex-col w-72 md:w-80 mx-auto">
      <h2 className="text-4xl font-bold mb-6 mt-3 text-center">Sign Up</h2>
      {accountCreated ? (
        <>
          <div className="text-center font-semibold  mt-2">
            Please check your inbox for login link.
            {/* TODO: how long does it take for a login link to expire? */}
          </div>
          <Link href={`/account-created`}>
            <a
              className="
              flex
              font-semibold
              justify-center
              items-center
              mt-24
              text-blue-600
              "
            >
              (Sign up email template{' '}
              <ExternalLink className="mx-1" size={16} />)
            </a>
          </Link>
        </>
      ) : (
        <SignUpFields setAccountCreated={setAccountCreated} />
      )}
    </Layout>
  )
}
