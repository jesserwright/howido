import React, { useState } from 'react'

const CreateStep = () => {
  const [file, setFile] = useState<File | null>(null)

  function handleFileInput(event: React.ChangeEvent<HTMLInputElement>) {
    // The input does not have `multiple` so only one can be chosen.
    const files = event.currentTarget.files
    if (files == null || files.length !== 1) {
      console.error('File input errror')
    } else {
      const file = files[0]
      setFile(file)
    }
  }

  //   Does this need to be async await? No. But why ? Doesn't fetch return a promise?
  //   that might be because javascript promises are eager - evaluated. The file starts getting sent right away. The action is taken.
  // however, it is not awaited before function return, so perhaps a response is never received. check the network tab
  async function handleImageUpload() {
    const formData = new FormData()
    formData.append('myFile', file!) // we can assume the file exists because sendfile cannot be clicked unless there is one
    try {
      const resp = await fetch('http://localhost:3001/img-upload', {
        method: 'POST',
        body: formData,
      })
      const txt = await resp.text()
      console.log(txt)
    } catch (e) {
      console.error(e)
    }
  }

  //   if the server has to compress before viewing, that's lag. But the browser would otherwise have to keep the file around in memory.

  return (
    <div>
      <input type="file" onChange={handleFileInput} />
      {file !== null && (
        <button type="button" onClick={handleImageUpload}>
          Send File
        </button>
      )}
    </div>
  )
}

export default CreateStep
