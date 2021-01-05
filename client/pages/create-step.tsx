import React, { useState } from 'react'
import { v4 as uuidv4 } from 'uuid'

// Do you need a reverse proxy? is there *any* way to avoid that moving part? It is a pretty serious pain.
// Can DNS 'be my reverse proxy'? (host the api on a subdomain like `api.`) That would mean separate ip addresses. And separate systems No problem with that?
// I would prefer to keep everything built into the binary. Never mind that right now. Just one simple goal:
// TODO: open two ports on my mac, so that I can upload with the phone. That is the goal. See how the upload looks on my phone.
// This is also generally useful for web dev on macs into the future. Document it.

const CreateStep = () => {
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
    const fileSizeMb = file.size / 1000 / 1024 // not sure if this is exact math to the mb, but it's aprox
    const MAX_SIZE_MB = 4
    if (fileSizeMb > MAX_SIZE_MB) {
      // This is a user input error message
      setErrorMsg(`Image size is too big. Must be smaller than ${MAX_SIZE_MB}`)
      return
    }

    // Validate file type
    if (!file['type'].includes('image')) {
      setErrorMsg('Image format not supported.')
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
    // What should the file extension be???? shal it be preserved?
    // const fileName = uuidv4() + '.jpg'
    formData.append('stepImage', props.file) // we can assume the file exists because sendfile cannot be clicked unless there is one
    try {
      const resp = await fetch('http://localhost:3001/img-upload', {
        method: 'POST',
        body: formData,
      })
      const txt = await resp.text()
      console.log(txt)
    } catch (e) {
      // Behold, another error
      // This would be a system error at this point, because the file
      // is already checked for size.
      console.error(e)
    }
    // Await should happen, because it does matter if it was successful or not
  }

  return (
    <button type="button" onClick={handleImageUpload}>
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
