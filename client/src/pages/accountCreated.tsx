import React from 'react'
import { Link } from 'react-router-dom'
import { ChevronRight } from 'react-feather'

export default function EmailTemplate() {
  // Remember that the email template will be self-contained, and in a separate place .
  return (
    <div className="bg-gray-100 h-screen">
      <div className="p-6 flex flex-col items-start justify-center sm:rounded-xl container mx-auto">
        <h1 className="text-2xl mb-2">
          <span className="font-bold">How I Do</span>
          &nbsp;&nbsp;|&nbsp;&nbsp;Confirmation
        </h1>
        <p className="my-3">
          Just making sure it's you, <b>Jesse Wright</b>.
        </p>
        <Link to="/profile">
          <a className="text-blue-600 flex-row flex items-center">
            <span>Login</span>
            <ChevronRight size={16} className="ml-0.5" />
          </a>
        </Link>
        <hr className="my-5 w-full" />
        <h2 className="mb-2">
          <span>Your Password:</span>
          <span className="font-semibold font-mono mb-2 tracking-wider bg-white py-1 px-2 rounded-md ml-2">
            $NcFwTHrn4
          </span>
        </h2>
        <a href="/" className="text-blue-600 flex-row flex items-center">
          Change password <ChevronRight size={16} className="ml-0.5" />
        </a>
      </div>
    </div>
  )
}
