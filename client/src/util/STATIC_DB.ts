export enum PointType {
  Warn,
  KeyPoint,
}

type PointProps = {
  id: number
  title: string
  pointType: PointType
}

export type StepProps = {
  id: number
  title: string
  imageFilename: string
  points: PointProps[]
}

export type HowToProps = {
  id: number
  title: string
  notes: string[]
  author: User
  steps: StepProps[]
}

export type User = {
  id: number
  name: string
}

export const USER = {
  id: 1,
  name: 'Jesse Wright',
}

export const USERS = [
  // USER, not for now...
  { id: 2, name: 'Chuck Chesterton' },
  { id: 3, name: 'Ernie MacEntire' },
  { id: 4, name: 'Robby McRobinson' },
  { id: 5, name: 'Frank Frankenstien' },
]

// TODO: do something about hours
// TODO: extract this and make another one for content-overflow testing
// the route will be something like: `how-to/_how_to_id_`
export const HOWTOS: HowToProps[] = [
  {
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
        imageFilename: 'IMG_2665.webp',
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
        imageFilename: 'IMG_2710.webp',
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
        imageFilename: 'IMG_2666.webp',
        points: [],
      },
      {
        id: 4,
        title: 'Unlatch lid-stop',
        imageFilename: 'IMG_2667.webp',
        points: [],
      },
      {
        id: 5,
        title: 'Rest on upper lid-stop',
        imageFilename: 'IMG_2668.webp',
        points: [],
      },
      {
        id: 6,
        title: 'Remove water from hanging position',
        imageFilename: 'IMG_2669.webp',
        points: [],
      },
      {
        id: 7,
        title: 'Fill Water',
        imageFilename: 'IMG_2670.webp',
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
        imageFilename: 'IMG_2671.webp',
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
        imageFilename: 'IMG_2712.webp',
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
        imageFilename: 'IMG_2684.webp',
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
        imageFilename: 'IMG_2672.webp',
        points: [
          {
            id: 12,
            title: 'Put your legs into it!',
            pointType: PointType.KeyPoint,
          },
        ],
      },
    ],
  },
  {
    id: 2,
    author: { id: 2, name: 'Chuck Chesterton' },
    notes: ['Note'],
    title: 'Chicken Soups',
    steps: [],
  },
]
