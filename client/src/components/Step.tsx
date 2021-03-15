import React, { useState } from 'react'
import { StepProps, PointType } from '../util/STATIC_DB'

export function Step(props: StepProps) {
  // the state of the input. Will be in separate component.
  const { id, title, imageFilename } = props
  let path = '//localhost:80/api/images/' + imageFilename
  // Log the filename
  // useEffect(() => {
  //   fetch(path)
  //     .then((val) => val.blob)
  //     .then(console.log)
  // })
  return (
    <div
      className="
        rounded-lg
        shadow
        flex
        border
        sm:flex-row
        flex-col-reverse
        bg-white
        mb-8
        sm:h-80
        "
    >
      {/* "picture" can offer different sources depending on availability (like avif/webp) as well as screen width */}
      {/* <picture> */}
      <img
        src={path}
        alt=""
        className="rounded-b-lg sm:rounded-l-lg sm:rounded-r-none w-full sm:w-80"
      />
      {/* </picture> */}
      <div className="flex flex-col pt-4 pl-4 sm:pt-3 w-full">
        <StepTitle title={title} />
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

const StepTitle = (props: { title: string }) => {
  const [open, setOpen] = useState(false)
  const [title, setTitle] = useState(props.title)
  function toggleUpdateStep() {
    setOpen(!open)
  }
  function updateTitle(event: React.ChangeEvent<HTMLInputElement>) {
    setTitle(event.target.value)
    // make the request to update the step.
  }

  function saveTitleUpdate() {
    console.log(`updated: ${title}`)
    setOpen(false)
  }

  function handleKeyDown(event: React.KeyboardEvent) {
    if (event.key === 'Enter') {
      setOpen(false)
    }
  }

  if (!open) {
    return (
      <h3
        onClick={toggleUpdateStep}
        className="text-xl font-medium pr-auto pr-1 leading-tight cursor-pointer border-black border-2"
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
