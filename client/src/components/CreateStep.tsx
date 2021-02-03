import React, { useState } from 'react'
import { HOWTOS } from '../util/STATIC_DB'

const CreateStep = () => {
  // it should not be possible for there to be a file and an error - that is an invalid state
  // There is either an error or a file, not both
  const [file, setFile] = useState<File | null>(null)
  const [errorMsg, setErrorMsg] = useState<string | null>(null)

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
      setErrorMsg('File format not supported.')
      return
    }

    setFile(file)
  }

  return (
    <div>
      {/* The input does not have `multiple` so only one can be chosen. */}
      <input type="file" onChange={handleFileInput} />
      {file && (
        // Only show the submit if the file exists
        <SendImgButton file={file} />
      )}
      {errorMsg && <div>{errorMsg}</div>}
    </div>
  )
}

const SendImgButton = (props: { file: File }) => {
  async function handleImageUpload() {
    const formData = new FormData()
    // Another field can be added - the howto_id and title. All at once?
    // Are images optional?
    // What should the file extension be???? shal it be preserved?
    // const fileName = uuidv4() + '.jpg'

    // The base problems:
    // 1. I don't know exactly what content disposition is
    // 2. I don't know how I want the software to behave.
    // ...this is how all files on the web are uploaded...

    // Other: I would prefer JSON over this form business.

    const HOWTO_ID = '1'
    formData.append('title', 'Step title')
    formData.append('howToId', HOWTO_ID)
    formData.append('image', props.file) // we can assume the file exists because sendfile cannot be clicked unless there is one

    // "CANCEL" on the file input causes a file input error - when it should probably not.

    try {
      // Hardcoded API. No bueno. Use environment.
      const resp = await fetch('http://localhost:3001/img-upload', {
        method: 'POST',
        body: formData,
      })
      // Technically, this could explode. (Deserializing can explode)
      const r = await resp.json()
      console.log(r)
    } catch (e) {
      // Behold, another error
      // This would be a system error at this point, because the file
      // is already checked for size.
      console.error(e)
    }
    // file can also be zipped... but if it's cropped first that's not a problem
    // Gotta make that choice with math and reason.
    // Image upload is *the* name of the game here.
    // This can be changed in the future fairly easily. It doesn't get embedded into the app that much.

    // Await should happen, because it does matter if it was successful or not
  }

  return (
    <button
      type="button"
      className="border rounded bg-red-200 py-2 px-4"
      onClick={handleImageUpload}
    >
      Send File
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
