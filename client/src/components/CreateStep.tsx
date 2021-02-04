import React, { useRef, useState } from 'react'
import {Save} from 'react-feather'
import { HOWTOS } from '../util/STATIC_DB'

const CreateStep = () => {
  // it should not be possible for there to be a file and an error - that is an invalid state
  // There is either an error or a file, not both
  const [file, setFile] = useState<File | null>(null)
  const [fileSrc, setFileSrc] = useState<string | null>(null)
  const [errorMsg, setErrorMsg] = useState<string | null>(null)

  const fileInputRef = useRef<HTMLInputElement>(null)

  function simulateClick() {
    if (fileInputRef.current != null) {
      fileInputRef.current.click()
    }
  }

  function handleFileInput(event: React.ChangeEvent<HTMLInputElement>) {
    const files = event.currentTarget.files

    // Validate Input
    if (files == null || files.length !== 1) {
      // this is a system / os error
      setErrorMsg('File input error')
      return
    }

    // Validate Size
    const file = files[0]

    console.log(file['type'])
    fetch(`/api/${file['type']}`)
      .then((resp) => resp.text())
      .then(console.log)

    // TODO: math. what hapens when you divide by a number twice?
    const fileSizeMb = file.size / 1024000 // not sure if this is exact math to the mb, but it's aprox
    const MAX_SIZE_MB = 4
    if (fileSizeMb > MAX_SIZE_MB) {
      // This is a user input error message
      setErrorMsg(`Image size is too big. Must be smaller than ${MAX_SIZE_MB}`)
      return
    }

    // Validate file type
    if (!file['type'].includes('image')) {
      // TODO: only support image/png, image/jpg? Is heif on the iPhone?
      // how can we get that kind of information?
      setErrorMsg('File format not supported.')
      return
    }

    setFileSrc(URL.createObjectURL(file))
    setFile(file)
  }

  // Navigator.getUserMedia() might be a better api so the user doesn't have to leave the browswer? Maybe.
  return (
    <div>
      {/* The input does not have `multiple` so only one can be chosen. */}
      {/* TODO: hide this input and use a button to trigger it. (default is unsightly). Will require an artificial click. */}
      <input
        ref={fileInputRef}
        type="file"
        accept="image/*"
        capture="environment" // capture causes the camera to open. This is preferred for now. Desktop just choses img.
        placeholder="Add Photo"
        onChange={handleFileInput}
        style={{ display: 'none' }}
      />
      {!file && (
        <button type="button" onClick={simulateClick}>
          + Photo
        </button>
      )}
      {file && (
        // How can the image be shown bigger before it is sent?
        <>
          <img src={`${fileSrc}`} alt="" />
          <SendImgButton file={file} />
        </>
      )}
      {errorMsg && <div>{errorMsg}</div>}
    </div>
  )
}

const SendImgButton = (props: { file: File }) => {
  async function handleImageUpload() {
    const formData = new FormData()
    const HOWTO_ID = '1'
    formData.append('title', 'Step title')
    formData.append('howToId', HOWTO_ID)
    formData.append('image', props.file) // we can assume the file exists because sendfile cannot be clicked unless there is one

    // "CANCEL" on the file input causes a file input error - when it should probably not.

    try {
      const resp = await fetch('http://localhost:3001/img-upload', {
        method: 'POST',
        body: formData,
      })
      // Technically, this could explode. (Deserializing can explode. io-ts can help with this if I want to go that way - safe decode of a structure)
      const r = await resp.json()
      console.log(r)
    } catch (e) {
      console.error(e)
    }
  }

  return (
    <button
      type="button"
      className="border rounded bg-gray-300 py-3 px-5 mt-2 flex font-bold ml-auto"
      onClick={handleImageUpload}
    >
      Save
      <Save className="ml-2" />
    </button>
  )
}

export default CreateStep

// TODO: display user level input errors, and system errors
// 1. Client side input error (prevent as much as possible automatically. Like prevent typing past the char limmit like ebay does. add a countdown only on the last 10 chars)
// 2. Server side input error (this can't be checked on the client side)
// 3. Client side exception (not shown to user? Or it is, and a 'How I Do client crashed'. console.error this for dbg purpues. Then a green button: 'Report error and reload')
// This will send the information reguarding the exception, and reload the page
// the error report may take a maximum of 2 seconds before force-relading the page (set timeout that runs a reload function)

// Other: no field level error stuff. This system is fine-grained and per-field. There are not many input fields, so some duplication might be OK.

// user error vs system error... input/application validation VS the mechine had a problem. Don't think in terms of libs, think in terms of structures.

// const ErrorDialog = () => {
//   return (
//     <div>
//       <h1>{errorMsg}</h1>
//     </div>
//   )
// }
