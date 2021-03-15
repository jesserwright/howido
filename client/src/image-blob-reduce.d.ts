// Use explicit any types for pica, because typescript is not supported.

declare module 'image-blob-reduce' {
  export default ImageBlobReduce
  export var pica: any
}
