export const outputFileEndings = {
  'image/jpeg': 'jpeg',
  'image/png': 'png',
  'image/webp': 'webp',
  'image/gif': 'gif',
  'image/bmp': 'bmp',
  'image/tiff': 'tiff',
  'image/x-icon': 'ico',
  'image/avif': 'avif',
  'image/farbfeld': 'ff',
  'image/vnd.radiance': 'hdr',
  'image/x-exr': 'exr',
  'image/x-qoi': 'qoi',
  'image/x-targa': 'tga',
  'image/x-portable-anymap': 'pnm',
}

export const inputFileEndings = {
  ...outputFileEndings,
  'image/svg+xml': 'svg',
}

export const acceptList = [
  'image/*',
  // Farbfeld
  '.ff',
  // HDR
  '.hdr',
  //
].join(',')

export type MimeTypes = keyof typeof outputFileEndings | keyof typeof inputFileEndings | 'application/octet-stream'

export function getMimeType(file: File): MimeTypes {
  if (file.type === '') {
    const extension = file.name.split('.').pop()

    console.log('extension', extension)
    console.log(Object.entries(inputFileEndings))

    if (extension) {
      Object.entries(inputFileEndings).forEach(([mimeType, ext]) => {
        if (ext === extension) {
          console.log('mimeType', mimeType)
          return mimeType
        }
      })
    }

    return 'application/octet-stream'
  }

  return file.type as keyof typeof outputFileEndings
}
