import Layout from '../components/Layout'
import { USER, USERS } from '../util/STATIC_DB'
import React from 'react'

// TODO: make this route dependent on the user id
export default function Followers() {
  return (
    <Layout pageTitle={`${USER.name}'s Followers`}>
      <div>
        <h1 className="text-2xl">{`${USER.name}'s Followers`}</h1>
        <ul>
          {USERS.map((person) => (
            <li key={person.id}>{person.name}</li>
          ))}
        </ul>
      </div>
    </Layout>
  )
}
