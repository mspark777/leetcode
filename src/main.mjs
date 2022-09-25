class MyCircularQueue {
  /**
   * @param {number} k
   */
  constructor (k) {
    /** @type number[] */
    this.queue = new Array(k)
    this.begin = 0
    this.end = 0
    this.size = 0
  }

  /**
   * @param {number} value
   * @returns {boolean}
   */
  enQueue (value) {
    if (this.isFull()) {
      return false
    }

    const end = this.end
    this.queue[end] = value
    this.end = this.#nextIndex(end)
    this.size += 1
    return true
  }

  /**
   * @returns {boolean}
   */
  deQueue () {
    if (this.isEmpty()) {
      return false
    }

    this.begin = this.#nextIndex(this.begin)
    this.size -= 1
    return true
  }

  /**
   * @returns {number}
   */
  Front () {
    return this.isEmpty() ? -1 : this.queue[this.begin]
  }

  /**
   * @returns {number}
   */
  Rear () {
    if (this.isEmpty()) {
      return -1
    }

    const end = this.end
    const queue = this.queue
    const tail = end === 0 ? queue.length - 1 : end - 1
    return queue[tail]
  }

  /**
   * @returns {boolean}
   */
  isEmpty () {
    return this.size < 1
  }

  /**
   * @returns {boolean}
   */
  isFull () {
    return this.size >= this.queue.length
  }

  /**
   * @param {number} cur
   * @returns {number}
   */
  #nextIndex (cur) {
    return (cur + 1) % this.queue.length
  }
}

async function main () {
  const queue = new MyCircularQueue(3)
  console.log(queue.enQueue(1))
  console.log(queue.enQueue(2))
  console.log(queue.enQueue(3))
  console.log(queue.enQueue(4))
  console.log(queue.Rear())
  console.log(queue.isFull())
  console.log(queue.deQueue())
  console.log(queue.enQueue(4))
  console.log(queue.Rear())
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
