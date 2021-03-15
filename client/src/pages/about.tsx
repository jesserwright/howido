import React from 'react'
import Layout from '../components/Layout'

export default function About() {
  return (
    <Layout pageTitle="About" className="prose md:prose-lg">
      <h1 className="">About</h1>
      <h2>Mission</h2>
      <blockquote>
        Empower everyone to share what they do with the world
      </blockquote>
      <h2>We value the open web</h2>
      <p>
        We want <b>How I Do</b> to be valuable to everyone - including those we
        do not financially bennefit from. Our software is useful without
        creating an account, and we plan to stay that way. For example, we don't
        require that you create an account in order to view how-tos.
      </p>

      <h2>Excellence</h2>
      <p>
        We want the vast majority of the things we do to be <sup>9</sup>&frasl;
        <sub>10</sub> quality. This includes areas such as communication and
        technical decisions. While we aim at excellence, we try to keep it in
        tension with our mission - to deliver useful software in a timely
        manner. A concrete example of this: when faced with the decision to add
        another feature or fix a bug, we will fix the bug!
      </p>

      <h2>Inclusion</h2>
      <p>
        We believe that inclusion and acceptance should be our attitude to
        others, especially to those different than us.
      </p>
      <p>
        One way we aim to realize this value is through the internationalization
        of our software. We realize much of the world does not speak english as
        a first language, and we want to reach as many people as we can.
      </p>
      <hr className="my-6 border-none" />
    </Layout>
  )
}
