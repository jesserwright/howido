import React, { useEffect, useState } from 'react'
import Layout from '../components/Layout'

export default function Home() {
  const [text, setText] = useState('')
  useEffect(() => {
    const resp = fetch('api/hello', {})
      .then((resp) => resp.text())
      .then((data) => {
        setText(data)
      })
  }, [])

  return (
    <Layout pageTitle="Create How Tos">
      <h1 className="text-4xl md:text-9xl font-black flex flex-col">
        CREATE HOW TOS
      </h1>
      <div className="text-center flex justify-center items-center my-8">
        <span className="text-9xl">[</span>
        <div className="mt-1">
          Home / Marketing page / auto play video of the awesome things how i do
          has on it. Featured / popular users and content.
        </div>
        <span className="text-9xl">]</span>
        {text}
      </div>
    </Layout>
  )
}
