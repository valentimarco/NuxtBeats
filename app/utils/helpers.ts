import type { Error as TauriError } from "./tauri"

/**
 * Catches errors from a promise.
 * @param promise The promise to handle.
 * @param options Additional options for handling the promise.
 * @returns A tuple with either the error or the result of the promise.
 * @throws Will rethrow the error if it is not in the `errorsToCatch` array.
 */
export async function tryCatch<T = any, E extends new (...args: any[]) => Error = ErrorConstructor>(
  promise: Promise<T>,
  options?: {
    /** An optional array of error constructors to catch */
    errorsToCatch?: E[]
    /** An optional message to log when an error occurs */
    logMessage?: string
    /** A callback function to execute on success */
    onSuccess?: (result: T) => void
    /** A callback function to execute on error */
    onError?: (error: E) => void
  },
): Promise<[undefined, T] | [InstanceType<E>]> {
  const { errorsToCatch, logMessage, onError, onSuccess } = options ?? {}
  try {
    const res = await promise
    onSuccess?.(res)
    return [undefined, res]
  }
  catch (error: any) {
    if (errorsToCatch === undefined || errorsToCatch.some(e => error instanceof e)) {
      console.error(logMessage || "An error occurred while executing a promise:")
      console.dir(error)
      onError?.(error)
      return [error]
    }
    throw error
  }
}
