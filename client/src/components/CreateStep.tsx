import React, { useContext, useRef, useState } from 'react'
import { PlusCircle, Camera, X as XIcon, Check } from 'react-feather'
import { AppContext } from '../App'
import { mutate } from 'swr'

type TPickedFile = { file: File; fileURL: string } | null

const CreateStep = () => {
  const [open, setOpen] = useState(false)
  const [pickedFile, setPickedFile] = useState<TPickedFile>(null)
  const [title, setTitle] = useState<string>('')
  const { setErrorMessage, serverError: errorMessage } = useContext(AppContext)
  const [loading, setLoading] = useState(false)

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

  const fileInputRef = useRef<HTMLInputElement>(null)
  function simulateClick() {
    if (fileInputRef.current != null) {
      fileInputRef.current.click()
    }
  }

  async function createStep(file: File) {
    const HOW_TO_ID = '1'

    const formData = new FormData()
    formData.append('title', title)
    formData.append('howToId', HOW_TO_ID)
    formData.append('image', file)

    try {
      setLoading(true)
      const resp = await fetch(`${import.meta.env.API_URL}/img-upload`, {
        method: 'POST',
        body: formData,
      })
      try {
        const jsonResponse = await resp.json()
        console.log(jsonResponse)
        setLoading(false)
      } catch (error) {
        // failed to parse response to json
      }
    } catch (e) {
      // network related failure
    }

    mutate(`/how-to/${HOW_TO_ID}`)
    setPickedFile(null)
    setOpen(false)
  }
  return (
    <div className="flex flex-col">
      {open ? (
        <>
          <XIcon
            onClick={() => {
              setOpen(false)
              setPickedFile(null)
              setTitle('')
            }}
            className="ml-auto mb-2 cursor-pointer"
          />
          <input
            className="rounded"
            type="text"
            placeholder="Title"
            onChange={handleTitleInput}
          />
          {pickedFile ? (
            <>
            {/* this is the image tag that needs 1:1 overylay */}
              <img
                className="rounded mt-2"
                src={pickedFile.fileURL}
                alt={title}
              />
              <button
                type="button"
                className="border rounded bg-gray-300 py-3 px-5 mt-2 font-bold w-full"
                onClick={() => createStep(pickedFile.file)}
              >
                {loading ? (
                  'Uploading...'
                ) : (
                  <Check size={32} className="m-auto" />
                )}
              </button>
            </>
          ) : (
            <>
              <input
                ref={fileInputRef}
                type="file"
                accept="image/jpeg"
                capture="environment"
                onChange={handleFileInput}
                style={{ display: 'none' }}
              />
              <button className="ml-auto" type="button" onClick={simulateClick}>
                <Camera className="mt-2" />
              </button>
            </>
          )}
        </>
      ) : (
        <PlusCircle
          className="ml-auto cursor-pointer"
          onClick={() => setOpen(true)}
        />
      )}
      {errorMessage && <div>{errorMessage}</div>}
    </div>
  )
}

export default CreateStep
