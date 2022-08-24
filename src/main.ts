class MyQueue {
  private readonly current: number[]
  private readonly buffer: number[]
  constructor () {
    this.current = []
    this.buffer = []
  }

  push (x: number): void {
    this.buffer.push(x)
  }

  pop (): number {
    this.fillFromBuffer()
    return this.current.pop() as number
  }

  peek (): number {
    this.fillFromBuffer()
    return this.current.at(-1) as number
  }

  empty (): boolean {
    const size = this.current.length + this.buffer.length
    return size < 1
  }

  private fillFromBuffer (): void {
    const current = this.current
    const buffer = this.buffer
    if (current.length < 1) {
      while (buffer.length > 0) {
        current.push(buffer.pop() as number)
      }
    }
  }
}

async function main (): Promise<void> {
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
