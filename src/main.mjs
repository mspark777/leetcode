class MyQueue {
  constructor () {
    this.current = []
    this.buffer = []
  }

  /**
   * @param {number} x
   * @returns {void}
  */
  push (x) {
    this.buffer.push(x)
  }

  /**
   * @returns {number}
   */
  pop () {
    this.#fillFromBuffer()
    return this.current.pop()
  }

  /**
   * @returns {number}
   */
  peek () {
    this.#fillFromBuffer()
    return this.current.at(-1)
  }

  /**
   * @returns {boolean}
   */
  empty () {
    const size = this.current.length + this.buffer.length
    return size < 1
  }

  #fillFromBuffer () {
    const current = this.current
    const buffer = this.buffer
    if (current.length < 1) {
      while (buffer.length > 0) {
        current.push(buffer.pop())
      }
    }
  }
}

async function main () {
  const queue = new MyQueue()
  queue.push(1)
  queue.push(2)
  console.log(queue.peek())
  console.log(queue.pop())
  console.log(queue.empty())
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
