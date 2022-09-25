class MyCircularQueue {
  private readonly queue: number[]
  private begin: number
  private end: number
  private size: number
  constructor (k: number) {
    this.queue = new Array<number>(k)
    this.begin = 0
    this.end = 0
    this.size = 0
  }

  enQueue (value: number): boolean {
    if (this.isFull()) {
      return false
    }

    const end = this.end
    this.queue[end] = value
    this.end = this.nextIndex(end)
    this.size += 1
    return true
  }

  deQueue (): boolean {
    if (this.isEmpty()) {
      return false
    }

    this.begin = this.nextIndex(this.begin)
    this.size -= 1
    return true
  }

  Front (): number {
    return this.isEmpty() ? -1 : this.queue[this.begin]
  }

  Rear (): number {
    if (this.isEmpty()) {
      return -1
    }

    const end = this.end
    const queue = this.queue
    const tail = end === 0 ? queue.length - 1 : end - 1
    return queue[tail]
  }

  isEmpty (): boolean {
    return this.size < 1
  }

  isFull (): boolean {
    return this.size >= this.queue.length
  }

  private nextIndex (cur: number): number {
    return (cur + 1) % this.queue.length
  }
}

async function main (): Promise<void> {
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
