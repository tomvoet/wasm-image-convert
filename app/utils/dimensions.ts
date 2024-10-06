export class Dimensions {
  public width: number
  public height: number

  constructor(width: number, height: number) {
    this.width = width
    this.height = height
  }

  static fromSVGViewBox(viewBox: string): Dimensions {
    const [_x, _y, width, height] = viewBox.split(' ').map(Number.parseFloat)
    return new Dimensions(width, height)
  }

  public getAspectRatio(): number {
    return this.width / this.height
  }

  public getSVGData(): SVGData {
    return {
      width: this.width,
      height: this.height,
      aspectRatio: this.getAspectRatio(),
    }
  }
}

export interface SVGData {
  width: number
  height: number
  aspectRatio: number
}
