import type { Settings } from '~/utils/settings'

export enum WorkerMessageType {
  PROGRESS,
  DONE,
  ERROR,
}

export interface WorkerResponse {
  success: boolean
  data?: Uint8Array
  error?: string
}

export interface WorkerProgress {
  progress: number
  message: string
}

export type WorkerMessage = {
  type: WorkerMessageType.DONE
  payload: WorkerResponse
} | {
  type: WorkerMessageType.ERROR
  payload: WorkerResponse
} | {
  type: WorkerMessageType.PROGRESS
  payload: WorkerProgress
}

export interface WorkerRequest {
  inputFile: Uint8Array
  inputType: string
  outputType: string
  settings?: Settings
}
