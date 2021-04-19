import React, { useContext, useRef, useState } from 'react'
import { Save } from 'react-feather'
import { AppContext } from '../App'
import ExifReader from 'exifreader';

// Work on create step.
// That means look at the mockup on figma. Then build out the frame a little. That means some css.
/*
NOTES
- There should be a 'retake' button if not satisfactory. If it is, then crop to 2160 or 1080 and upload. Persist the source on the client.
- Image is backwards on ios.
*/
type TPickedFile = { file: File; fileURL: string } | null

const CreateStep = () => {
  // it should not be possible for there to be a file and an error - that is an invalid state
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
    const arrayBuffer = await file.arrayBuffer();
    const tags = ExifReader.load(arrayBuffer, {expanded: true});
    console.log(tags.exif?.Orientation)
    console.log('hi')


    // The file also needs to be cropped. "croppie" might be the way to do that. Can it be afixed to do 1:1 crop?
    // Is it a crop & compress in one step?

    // let reducer = new ImageBlobReduce({
    //   pica: ImageBlobReduce.pica({ features: ['js', 'wasm', 'ww'] }),
    // })

    // let reducedFile: File
    // try {
    //   reducedFile = await reducer.toBlob(file, {
    //     max: 500,
    //     unsharpAmount: 80,
    //     unsharpRadius: 0.6,
    //     unsharpThreshold: 2,
    //   })
    // } catch (error) {
    //   setErrorMessage({
    //     message: `failed to resize image: ${error}`,
    //     fieldName: 'Image Crop',
    //   })
    //   // Early returns are sketchy, because they're not verifiably correc
    //   return
    // }

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
      {/* Below several components might need to be their own component. */}
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
        + Photo
      </button>
    </>
  )
}

type StepCreateInput = {
  title: string
  image: File
  howToId: number
}

async function createStep(step: StepCreateInput) {
  const formData = new FormData()

  // Field names
  const TITLE = 'title'
  const HOW_TO_ID = 'howToId'
  const IMAGE = 'image'

  formData.append(TITLE, step.title)
  formData.append(HOW_TO_ID, step.howToId.toString())
  formData.append(IMAGE, step.image)

  try {
    const resp = await fetch('http://192.168.0.178:3000/api/img-upload', {
      method: 'POST',
      body: formData,
    })
    try {
      const jsonResponse = await resp.json()
      console.log(jsonResponse)
    } catch (error) {
      // failed to parse response to json
    }
  } catch (e) {
    // network related failure
  }
}

const SendImgButton = (props: { file: File; title: string }) => {
  // This is actually 'handle step create'
  const STEP = { image: props.file, title: props.title, howToId: 1 }
  function handleCreateStep() {
    createStep(STEP)
  }
  return (
    <button
      type="button"
      className="border rounded bg-gray-300 py-3 px-5 mt-2 font-bold w-full"
      onClick={handleCreateStep}
    >
      Save
      <Save className="ml-2" />
    </button>
  )
}

export default CreateStep
