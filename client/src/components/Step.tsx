import React, { useState } from 'react'
import { StepProps, PointType } from '../util/STATIC_DB'
import { Edit, Trash, Code } from 'react-feather'
import { mutate } from 'swr'

type StepComponentProps = {
  step: StepProps
  howToId: string
}
export function Step(props: StepComponentProps) {
  const { id, title, imageFilename } = props.step
  const path = `${import.meta.env.API_URL}/images/${imageFilename}`

  async function handleDeleteStep() {
    try {
      // use SWR instead
      await fetch(`${import.meta.env.API_URL}/step/${id}`, { method: 'DELETE' })
      // This will refetch the whole how to in order to revalidate
      mutate(`/how-to/${props.howToId}`)
      // good response run a function that removes the element from the list
    } catch (error) {
      // bad api response or network error
    }
  }

  return (
    <div className="rounded-lg shadow flex border sm:flex-row flex-col-reverse bg-white mb-7 sm:h-80">
      <img
        src={path}
        alt={title}
        className="rounded-b-lg sm:rounded-l-lg sm:rounded-r-none w-full sm:w-80"
      />
      <div className="flex flex-col pl-4 w-full">
        <div className="flex justify-between">
          <StepTitle title={title} id={id} />
          <div className="flex space-x-3 sm:space-x-2 border-l-2 border-b-2 p-2 rounded-bl-lg rounded-tr-lg self-start">
            {/* This should trigger reordering */}
            {/* Show some kind of symbol for placing the step somewhere else in the order */}
            <Code
              size={20}
              className="cursor-pointer transition-colors hover:text-yellow-400"
            />
            <Trash
              onClick={handleDeleteStep}
              className="hover:text-red-600 transition-colors cursor-pointer"
              size={20}
            />
            {/* Edit button might not be needed in edit mode actually */}
            <Edit
              size={20}
              className="cursor-pointer transition-colors hover:text-green-600"
            />
          </div>
        </div>
        <ul className="list-outside list-disc pl-5 overflow-auto mt-2 mb-3 pr-2">
          {[
            {
              id: 1,
              title: 'Take out chickens',
              pointType: PointType.KeyPoint,
            },
          ]
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

const StepTitle = (props: { title: string; id: number }) => {
  const [open, setOpen] = useState(false)
  const [title, setTitle] = useState(props.title)

  function toggleUpdateStep() {
    setOpen(!open)
  }

  function updateTitle(event: React.ChangeEvent<HTMLInputElement>) {
    setTitle(event.target.value)
  }

  async function saveTitleUpdate() {
    setOpen(false)
    // call the API
    try {
      // this request is not matching on ther server. why?
      const resp = await fetch(`${import.meta.env.API_URL}/step`, {
        method: 'PUT',
        body: JSON.stringify({ id: props.id, title }),
      })
      try {
        const data = await resp.json()
        console.log(data)
      } catch (error) {
        console.log('failed to parse JSON')
      }
    } catch (error) {
      console.log('failed to fetch')
    }
  }

  function handleKeyDown(event: React.KeyboardEvent) {
    // TODO: this behavior is not wanted, if there is an edit mode
    // edit mode makes it an input, and focuses on the input w/ a cursor
    if (event.key === 'Enter') {
      setOpen(false)
    }
  }

  if (!open) {
    return (
      <h3
        onClick={toggleUpdateStep}
        className="text-xl font-medium pr-1 leading-tight cursor-pointer pt-3"
      >
        {title}
      </h3>
    )
  } else {
    return (
      <>
        <input
          onKeyDown={handleKeyDown}
          autoFocus
          type="text"
          value={title}
          onChange={updateTitle}
        />
        <button onClick={saveTitleUpdate}>Save</button>
        <button onClick={toggleUpdateStep}>close</button>
      </>
    )
  }
}
