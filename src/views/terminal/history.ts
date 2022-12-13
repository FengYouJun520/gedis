export class History<T = string> {

  histories: T[]

  current: number

  depth?: number

  constructor(depth?: number) {
    this.histories = []
    this.current = 0
    this.depth = depth
  }

  isEmpty() {
    return this.histories.length === 0
  }

  push(value: T) {
    if (this.histories.length === this.depth) {
      return
    }
    const index = this.histories.findIndex(t => t === value)
    if (index !== -1) {
      this.histories.splice(index, 1)
    }
    this.histories.push(value)
    this.current = this.histories.length
  }

  prevHistroy() {
    if (this.isEmpty()) {
      return ''
    }

    if (this.current - 1 < 0) {
      this.current = 0
      return this.histories[this.current]
    }

    this.current--
    return this.histories[this.current]
  }

  nextHistroy() {
    if (this.isEmpty() || this.histories.length === 1){
      return ''
    }

    if (this.current + 1 >= this.histories.length) {
      this.current = this.histories.length
      return ''
    }

    this.current++
    return this.histories[this.current]
  }

}
