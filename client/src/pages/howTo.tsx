import { Link } from 'react-router-dom'
import { ChevronRight } from 'react-feather'
import Layout from '../components/Layout'
import { HowToProps, HOWTOS } from '../util/STATIC_DB'
import { Step } from '../components/Step'
import CreateStep from '../components/CreateStep'
import React from 'react'

export default function HowTo(props: { howto: HowToProps }) {
  const { id, notes, title, steps, author } = props.howto

  const totalSeconds = steps.reduce((acc, curr) => (acc += curr.seconds), 0)

  const seconds = totalSeconds % 60 // the remainder seconds
  const minutes = (totalSeconds - seconds) / 60 // the total minus the remainder divided by 60, for even minutes

  return (
    <Layout pageTitle={title}>
      {/* Title */}
      <h1 className="text-2xl md:text-3xl font-medium mb-3 md:mb-4">{title}</h1>
      <div className="mb-3.5 inline-block text-sm md:text-base">
        ⏱ <strong>{minutes}</strong>m <strong>{seconds}</strong>
        s&nbsp;&nbsp;
        <span className="text-lg">|</span>&nbsp;&nbsp;
        <strong>{steps.length}</strong> Steps
      </div>

      {/* Author */}
      <div className="flex items-center text-sm md:text-base">
        <span className="text-gray-500">From&nbsp;</span>
        <Link
          to="/"
          className="group flex flow-row items-center hover:text-gray-500 transition-colors"
        >
          <span>{author.name}</span>
          <ChevronRight
            size={16}
            className="group-hover:translate-x-0.5 transform transition-transform"
          />
        </Link>
      </div>

      {/* Notes */}
      <div className="md:border-l-2 border-black md:pl-3 flex flex-col md:my-9 my-6">
        <h2 className="text-lg font-semibold mb-2">Notes</h2>
        <ul className="list-outside list-disc pl-5">
          {/* TODO: don't use map index for keys */}
          {notes.map((note, idx) => (
            <li key={idx}>{note}</li>
          ))}
        </ul>
      </div>

      {steps.map((step) => (
        <Step key={step.id} {...step} />
      ))}
      <CreateStep />
    </Layout>
  )
}
