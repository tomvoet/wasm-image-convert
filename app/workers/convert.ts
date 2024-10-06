import init, { convertImage } from '@@/wasm/pkg'
import { type WorkerMessage, WorkerMessageType, type WorkerRequest, type WorkerResponse } from './convert.d'

globalThis.addEventListener(
  'message',
  /*
  async (e: MessageEvent<WorkerRequest>) => {
    await init()

    try {
      const res = convertImage(
        e.data.inputFile,
        e.data.inputType,
        e.data.outputType,
        callback,
        e.data.settings,
      )

      const response: WorkerResponse = {
        success: true,
        data: res,
      }

      globalThis.postMessage({
        type: WorkerMessageType.DONE,
        payload: response,
      } as WorkerMessage)
    }
    catch (e) {
      const response: WorkerResponse = {
        success: false,
        error: String(e),
      }

      globalThis.postMessage({
        type: WorkerMessageType.ERROR,
        payload: response,
      })
    }
  },
  */
  (e: MessageEvent<WorkerRequest>) => {
    init().then(() => {
      const { inputFile, inputType, outputType, settings } = e.data

      const res = convertImage(inputFile, inputType, outputType, callback, settings)

      const response: WorkerResponse = {
        success: true,
        data: res,
      }

      globalThis.postMessage({
        type: WorkerMessageType.DONE,
        payload: response,
      } as WorkerMessage)
    }).catch((e) => {
      const response: WorkerResponse = {
        success: false,
        error: String(e),
      }

      globalThis.postMessage({
        type: WorkerMessageType.ERROR,
        payload: response,
      })
    })
  },
  false,
)

function callback(progress: number, message: string) {
  globalThis.postMessage({
    type: WorkerMessageType.PROGRESS,
    payload: {
      progress,
      message,
    },
  } as WorkerMessage)
}
