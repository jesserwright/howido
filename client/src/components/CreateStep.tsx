import React, { useContext, useRef, useState } from 'react'
import { PlusCircle, Camera, X as XIcon, Check, Loader } from 'react-feather'
import { AppContext } from '../App'
import { mutate } from 'swr'
import { Cropper as CropperReact } from 'react-cropper'
import 'cropperjs/dist/cropper.css'
import Cropper from 'cropperjs'
import ImageBlobReduce from 'image-blob-reduce'

type TPickedFile = { file: File; fileURL: string } | null

const HOW_TO_ID = '1'

const CreateStep = () => {
  const [open, setOpen] = useState(true)
  const [pickedFile, setPickedFile] = useState<TPickedFile>(null)
  const [title, setTitle] = useState<string>('')
  const { setErrorMessage, serverError: errorMessage } = useContext(AppContext)
  const [loading, setLoading] = useState(false)
  const cropperRef = useRef<HTMLImageElement>(null)
  const fileInputRef = useRef<HTMLInputElement>(null)

  function handleFileInput(event: React.ChangeEvent<HTMLInputElement>) {
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

  function simulateClick() {
    if (fileInputRef.current != null) {
      fileInputRef.current.click()
    }
  }

  function createStep() {
    setLoading(true)
    const imageElement: any = cropperRef?.current
    const cropper: Cropper = imageElement?.cropper
    const croppedCanvas = cropper.getCroppedCanvas()
    let reducedFile: File

    croppedCanvas.toBlob(async (blob) => {
      let reducer = new ImageBlobReduce({
        pica: ImageBlobReduce.pica({ features: ['js', 'wasm', 'ww'] }),
      })

      // maybe something more like this:
      // pica.resize(from, to)
      // .then(result => pica.toBlob(result, 'image/jpeg', 0.90))
      // .then(blob => console.log('resized to canvas & created blob!'));

      try {
        reducedFile = await reducer.toBlob(blob, { max: 640 })

        const formData = new FormData()
        formData.append('title', title)
        formData.append('howToId', HOW_TO_ID)
        formData.append('image', reducedFile)

        try {
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
        // upload from here instead of creating a url
        mutate(`/how-to/${HOW_TO_ID}`)
        setPickedFile(null)
        setOpen(false)
      } catch (error) {
        // setErrorMessage({
        //   message: `failed to resize image: ${error}`,
        //   fieldName: 'Image Crop',
        // })
        // Early returns are sketchy, because they're not verifiably correct
        return
      }
    })
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
            className="rounded mb-2"
            type="text"
            value={title}
            placeholder="Title"
            onChange={handleTitleInput}
          />
          {pickedFile ? (
            <>
              <CropperReact
                src={pickedFile.fileURL}
                // this needs to be non-responsive, and full
                style={{ maxWidth: 500, margin: 'auto' }}
                className="mt-2"
                // Cropper.js options
                aspectRatio={1}
                autoCropArea={1}
                viewMode={3}
                // https://github.com/fengyuanchen/cropperjs#checkorientation
                checkOrientation={false}
                initialAspectRatio={1}
                guides={false}
                // is this reference valid once the element has been created?
                ref={cropperRef}
              />
              {title && (
                <button
                  type="button"
                  className="border rounded bg-gray-300 py-3 px-5 mt-2 font-bold w-9"
                  onClick={createStep}
                >
                  {loading ? (
                    <Loader className="animate-spin m-auto" size={32} />
                  ) : (
                    <Check size={32} className="m-auto" />
                  )}
                </button>
              )}
            </>
          ) : (
            <>
              <input
                ref={fileInputRef}
                type="file"
                accept="image/jpeg"
                capture="environment"
                onChange={handleFileInput}
                className="hidden"
              />
              <button
                className="ml-auto focus:outline-none"
                type="button"
                onClick={simulateClick}
              >
                <Camera className="" />
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
