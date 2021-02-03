import Layout from '../components/Layout'
import { ChevronRight } from 'react-feather'
import { Link } from 'react-router-dom'
import React from 'react'
import { USER, USERS } from '../util/STATIC_DB'

// TODO: make this route dependent on the user id
export default function Following() {
  return (
    <Layout pageTitle={`${USER.name} is following`}>
      <h1 className="text-xl mb-4">
        <span className="font-bold">{USER.name}</span> is{' '}
        <span className="font-bold">Following</span>
      </h1>
      <ul className="">
        {USERS.map((user) => (
          <Link to={`/profile`}>
            <a
              key={user.id}
              className="flex items-center mb-4 border rounded-md bg-white shadow-sm hover:shadow p-2 group"
            >
              <img
                src="https://github.com/identicons/jasonlong.png"
                alt=""
                className="w-12 h-12 rounded-full border shadow-sm mr-4"
              />
              <li key={user.id} className="my-2 font-medium">
                {user.name}
              </li>
              <ChevronRight
                size={16}
                className={`group-hover:translate-x-0.5 transform transition-transform`}
              />
            </a>
          </Link>
        ))}
      </ul>
    </Layout>
  )
}
