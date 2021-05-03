import React, { useContext, useRef, useState } from 'react'
import { PlusCircle, Camera, X as XIcon, Check } from 'react-feather'
import { AppContext } from '../App'

type TPickedFile = { file: File; fileURL: string } | null

const CreateStep = () => {
  const [open, setOpen] = useState(false)
  const [pickedFile, setPickedFile] = useState<TPickedFile>(null)
  const [title, setTitle] = useState<string>('')
  const { setErrorMessage, serverError: errorMessage } = useContext(AppContext)

  async function handleFileInput(event: React.ChangeEvent<HTMLInputElement>) {
    const files = event.currentTarget.files

    if (files == null || files.length !== 1) {
      setErrorMessage({ fieldName: 'Step Image', message: 'File input error' })
      return
    }

    const file = files[0]
    const fileURL = URL.createObjectURL(file)
    setPickedFile({ fileURL, file: file })
  }

  function handleTitleInput(event: React.ChangeEvent<HTMLInputElement>) {
    setTitle(event.target.value)
  }

  return (
    <div className="flex flex-col">
      {/* If this is clicked, then show more */}
      {open ? (
        <>
          <XIcon
            onClick={() => {
              setOpen(false)
              setPickedFile(null)
              setTitle('')
            }}
            className="ml-auto mb-2"
          />
          <input
            className="rounded"
            type="text"
            placeholder="Title"
            onChange={handleTitleInput}
          />
          {pickedFile ? (
            <>
              <img className="rounded mt-2" src={pickedFile.fileURL} alt="" />
              <SendImgButton file={pickedFile.file} title={title} />
            </>
          ) : (
            <FileInput handleFileInput={handleFileInput} />
          )}
        </>
      ) : (
        <PlusCircle className="ml-auto" onClick={() => setOpen(true)} />
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
        <Camera className="mt-2" />
      </button>
    </>
  )
}

type StepCreateInput = {
  title: string
  image: File
  howToId: number
}

const SendImgButton = (props: { file: File; title: string }) => {
  const STEP = { image: props.file, title: props.title, howToId: 1 }
  const [loading, setLoading] = useState(false)

  async function createStep() {
    const formData = new FormData()

    // Field names
    const TITLE = 'title'
    const HOW_TO_ID = 'howToId'
    const IMAGE = 'image'

    formData.append(TITLE, STEP.title)
    formData.append(HOW_TO_ID, STEP.howToId.toString())
    formData.append(IMAGE, STEP.image)

    try {
      setLoading(true)
      const resp = await fetch(`${import.meta.env.API_URL}/img-upload`, {
        method: 'POST',
        body: formData,
      })
      try {
        const jsonResponse = await resp.json()
        // There should be a loading spinner here... it does take a while
        console.log(jsonResponse)
        setLoading(false)
      } catch (error) {
        // failed to parse response to json
      }
    } catch (e) {
      // network related failure
    }

    // Trigger a refetch of the whole "page data" AND close the edit thing
    // (while keeping scroll state, naturally)
  }

  return (
    <>
      <button
        type="button"
        className="border rounded bg-gray-300 py-3 px-5 mt-2 font-bold w-full"
        onClick={createStep}
      >
        {loading ? 'Uploading...' : <Check size={32} className="m-auto" />}
      </button>
    </>
  )
}

export default CreateStep
