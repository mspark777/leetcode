class MyStack {
  private readonly queue: number[]
  constructor () {
    this.queue = []
  }

  push (x: number): void {
    const queue = this.queue
    queue.push(x)

    const size = queue.length
    for (let i = 1; i < size; i += 1) {
      queue.push(queue.shift() as number)
    }
  }

  pop (): number {
    return this.queue.shift() as number
  }

  top (): number {
    return this.queue[0]
  }

  empty (): boolean {
    return this.queue.length < 1
  }
}

async function main (): Promise<void> {
  const myStack = new MyStack()
  myStack.push(1)
  myStack.push(2)
  console.log(myStack.top())
  console.log(myStack.pop())
  console.log(myStack.empty())
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
