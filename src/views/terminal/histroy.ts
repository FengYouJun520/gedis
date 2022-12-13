export class Histroy<T = string> {

  histroys: T[]

  current: number

  constructor() {
    this.histroys = []
    this.current = 0
  }

  isEmpty() {
    return this.histroys.length === 0
  }

  push(value: T) {
    const index = this.histroys.findIndex(t => t === value)
    if (index !== -1) {
      this.histroys.splice(index, 1)
    }
    this.histroys.push(value)
    this.current = this.histroys.length
  }

  prevHistroy() {
    if (this.isEmpty()) {
      return ''
    }

    if (this.current - 1 < 0) {
      this.current = 0
      return this.histroys[this.current]
    }

    this.current--
    return this.histroys[this.current]
  }

  nextHistroy() {
    if (this.isEmpty() || this.histroys.length === 1){
      return ''
    }

    if (this.current + 1 >= this.histroys.length) {
      this.current = this.histroys.length
      return ''
    }

    this.current++
    return this.histroys[this.current]
  }

}
