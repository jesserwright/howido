import Link from 'next/link'
import { ChevronRight } from 'react-feather'
import Layout from '../components/Layout'

enum PointType {
  Warn,
  KeyPoint,
}

type PointProps = {
  id: number
  title: string
  pointType: PointType
}

type StepProps = {
  id: number
  title: string
  seconds: number
  imageURI: string
  points: PointProps[]
}

type InstructionPageProps = {
  id: number
  title: string
  notes: string[]
  author: User
  steps: StepProps[]
}

type User = {
  id: number
  name: string
}

// TODO: do something about hours
// TODO: extract this and make another one for content-overflow testing
// the route will be something like: `how-to/_how_to_id_`
const STATIC_DB: InstructionPageProps = {
  id: 1,
  notes: [
    `Food, water, and moving pen must be done at least once every 3 days (if food and water are filled 100%)`,
    `Eggs must be collected every day`,
  ],
  author: { id: 1, name: 'Jesse Wright' },
  title: 'Morning Chicken Chores 🐓',
  steps: [
    {
      id: 1,
      title: 'Fill water bucket outside door and stuff',
      seconds: 20,
      imageURI: 'IMG_2665.webp',
      points: [
        {
          id: 1,
          title: 'Use the top nozzle (blue handle)',
          pointType: PointType.KeyPoint,
        },
      ],
    },
    {
      id: 2,
      title: 'Walk down to pen',
      seconds: 40,
      imageURI: 'IMG_2710.webp',
      points: [
        {
          id: 3,
          title: 'Set water and feed asside when you get there',
          pointType: PointType.KeyPoint,
        },
      ],
    },
    {
      id: 3,
      title: 'Unlatch carabiner',
      seconds: 5,
      imageURI: 'IMG_2666.webp',
      points: [],
    },
    {
      id: 4,
      title: 'Unlatch lid-stop',
      seconds: 4,
      imageURI: 'IMG_2667.webp',
      points: [],
    },
    {
      id: 5,
      title: 'Rest on upper lid-stop',
      seconds: 4,
      imageURI: 'IMG_2668.webp',
      points: [],
    },
    {
      id: 6,
      title: 'Remove water from hanging position',
      seconds: 15,
      imageURI: 'IMG_2669.webp',
      points: [],
    },
    {
      id: 7,
      title: 'Fill Water',
      seconds: 10,
      imageURI: 'IMG_2670.webp',
      points: [
        {
          id: 4,
          title: 'Rotate lid counter-clockwise to open, clockwise to close',
          pointType: PointType.KeyPoint,
        },
      ],
    },
    {
      id: 8,
      title: 'Fill feeder',
      seconds: 15,
      imageURI: 'IMG_2671.webp',
      points: [
        {
          id: 5,
          title: `Try not to spill it on the ground when pouring - that'll cause waste!`,
          pointType: PointType.KeyPoint,
        },
      ],
    },
    {
      id: 9,
      title: 'Collect Eggs',
      seconds: 5,
      imageURI: 'IMG_2712.webp',
      points: [
        {
          id: 6,
          title: 'They are usually in the corner where the water is',
          pointType: PointType.KeyPoint,
        },
        {
          id: 7,
          title: `Frequent collection means clean eggs (that don't need washing!)`,
          pointType: PointType.KeyPoint,
        },
      ],
    },
    {
      id: 10,
      title: 'Close lid and re-latch',
      seconds: 12,
      imageURI: 'IMG_2684.webp',
      points: [
        {
          id: 10,
          title: `Predators are vicious, persistent, and attentive. They will notice that it’s not latched 👀`,
          pointType: PointType.Warn,
        },
      ],
    },
    {
      id: 11,
      title: 'Move pen',
      seconds: 40,
      imageURI: 'IMG_2672.webp',
      points: [
        {
          id: 12,
          title: 'Put your legs into it!',
          pointType: PointType.KeyPoint,
        },
      ],
    },
  ],
}

function Step(props: StepProps) {
  const { id, imageURI, seconds, title, points } = props
  const imageURIRoot = process.env.imageURIRoot
  return (
    <div className="rounded-lg shadow flex border sm:flex-row flex-col-reverse bg-white mb-8 sm:h-80">
      <img
        src={imageURIRoot + imageURI}
        alt=""
        className="rounded-b-lg sm:rounded-l-lg sm:rounded-r-none w-full sm:w-80"
      />

      <div className="flex flex-col pt-4 pl-4 sm:pt-3 w-full">
        {/* BUG: extra div below for safari inconsistency */}
        <div>
          <div className="flex flex-row mr-4">
            <h3 className="text-xl font-medium mr-auto pr-1 leading-tight">
              {title}
            </h3>
            <div className="whitespace-nowrap text-sm mt-0.5 mr-0.5 ml-1">
              ⏱ <strong>{seconds}</strong>s
            </div>
          </div>
        </div>
        <ul className="list-outside list-disc pl-5 overflow-auto mt-2 mb-3 pr-2">
          {points
            .sort((a, b) => a.pointType - b.pointType)
            .map((point) => {
              const { id, pointType, title } = point
              if (pointType === PointType.Warn) {
                return (
                  <li key={id} className="text-red-600">
                    {title}
                  </li>
                )
              } else {
                return <li key={id}>{title}</li>
              }
            })}
        </ul>
      </div>
    </div>
  )
}

export default function InstructionPage(props: InstructionPageProps) {
  const { id, notes, title, steps, author } = STATIC_DB

  const totalSeconds = steps.reduce((acc, curr) => (acc += curr.seconds), 0)

  const seconds = totalSeconds % 60 // the remainder seconds
  const minutes = (totalSeconds - seconds) / 60 // the total minus the remainder divided by 60, for even minutes

  return (
      <Layout pageTitle={title}>
        {/* Title */}
        <h1 className="text-2xl md:text-3xl font-medium mb-3 md:mb-4">
          {title}
        </h1>
        <div className="mb-3.5 inline-block text-sm md:text-base">
          ⏱ <strong>{minutes}</strong>m <strong>{seconds}</strong>
          s&nbsp;&nbsp;
          <span className="text-lg">|</span>&nbsp;&nbsp;
          <strong>{steps.length}</strong> Steps
        </div>

        {/* Author */}
        <div className="flex items-center text-sm md:text-base">
          <span className="text-gray-500">From&nbsp;</span>
          <Link href="/">
            <a className="group flex flow-row items-center hover:text-gray-500 transition-colors">
              <span>{author.name}</span>
              <ChevronRight
                size={16}
                className="group-hover:translate-x-0.5 transform transition-transform"
              />
            </a>
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
      </Layout>
  )
}
