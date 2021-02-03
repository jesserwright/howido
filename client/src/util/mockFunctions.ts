// This is a mock. Throw in a type that you want back, and it will send it back 100ms later
export async function wait150ms<T>(obj: T, ms = 150) {
  return new Promise<T>((resolve, _reject) =>
    setTimeout(() => {
      resolve(obj)
    }, ms)
  )
}
