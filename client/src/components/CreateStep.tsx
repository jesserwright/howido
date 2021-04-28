import React, { useContext, useRef, useState } from 'react'
import { Save, PlusCircle, Camera } from 'react-feather'
import { AppContext } from '../App'

type TPickedFile = { file: File; fileURL: string } | null

const CreateStep = () => {
  // It should not be possible for there to be a file and an error - that is an invalid state
  // There is either an error or a file, not both
  // Having or null is not ideal. It is a lot of invalid states.
  // These both happen in the same function

  const [pickedFile, setPickedFile] = useState<TPickedFile>(null)
  const [title, setTitle] = useState<string>('')
  const { setErrorMessage, serverError: errorMessage } = useContext(AppContext)

  async function handleFileInput(event: React.ChangeEvent<HTMLInputElement>) {
    const files = event.currentTarget.files

    // Validate Input
    if (files == null || files.length !== 1) {
      setErrorMessage({ fieldName: 'Step Image', message: 'File input error' })
      return
    }

    const file = files[0]

    // Correct the exif rotation for iOS.
    // const arrayBuffer = await file.arrayBuffer()

    const fileURL = URL.createObjectURL(file)
    setPickedFile({ fileURL, file: file })
  }

  function handleTitleInput(event: React.ChangeEvent<HTMLInputElement>) {
    setTitle(event.target.value)
  }

  return (
    <div className="flex flex-col border">
      <h2>New Step</h2>
      <input type="text" placeholder="title" onChange={handleTitleInput} />
      {pickedFile ? (
        <>
          <img src={pickedFile.fileURL} alt="" />
          <SendImgButton file={pickedFile.file} title={title} />
        </>
      ) : (
        <FileInput handleFileInput={handleFileInput} />
      )}
      {errorMessage && <div>{errorMessage}</div>}
    </div>
  )
}

function FileInput(props: {
  handleFileInput(event: React.ChangeEvent<HTMLInputElement>): Promise<void>
}) {
  const fileInputRef = useRef<HTMLInputElement>(null)
  function simulateClick() {
    if (fileInputRef.current != null) {
      fileInputRef.current.click()
    }
  }
  return (
    <>
      <input
        ref={fileInputRef}
        type="file"
        accept="image/jpeg"
        capture="environment"
        onChange={props.handleFileInput}
        style={{ display: 'none' }}
      />
      <button className="ml-auto" type="button" onClick={simulateClick}>
        <Camera />
      </button>
    </>
  )
}

type StepCreateInput = {
  title: string
  image: File
  howToId: number
}

async function createStep(step: StepCreateInput, setLoading: () => void) {
  const formData = new FormData()

  // Field names
  const TITLE = 'title'
  const HOW_TO_ID = 'howToId'
  const IMAGE = 'image'

  formData.append(TITLE, step.title)
  formData.append(HOW_TO_ID, step.howToId.toString())
  formData.append(IMAGE, step.image)

  try {
    setLoading()
    const resp = await fetch(`${import.meta.env.API_URL}/img-upload`, {
      method: 'POST',
      body: formData,
    })
    try {
      const jsonResponse = await resp.json()
      // There should be a loading spinner here... it does take a while
      console.log(jsonResponse)
    } catch (error) {
      // failed to parse response to json
    }
  } catch (e) {
    // network related failure
  }
}

const SendImgButton = (props: { file: File; title: string }) => {
  const STEP = { image: props.file, title: props.title, howToId: 1 }
  const [loading, setLoading] = useState(false)
  function handleCreateStep() {
    createStep(STEP, () => setLoading(true))
  }
  return (
    <>
      <button
        type="button"
        className="border rounded bg-gray-300 py-3 px-5 mt-2 font-bold w-full"
        onClick={handleCreateStep}
      >
        Save
        {loading && 'LOADING'}
      </button>
      <button
        type="button"
        className="border rounded bg-gray-300 py-3 px-5 mt-2 font-bold w-full"
        onClick={handleCreateStep}
      >
        Choose Different
      </button>
    </>
  )
}


export default CreateStep
