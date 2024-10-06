export interface Settings {
  type: string
}

export interface SvgSettings extends Settings {
  type: 'svg'
  width: number
  height: number
}
