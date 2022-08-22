class MyStack {
  constructor () {
    this.queue = []
  }

  /**
 * @param {number} x
 * @return {void}
 */
  push (x) {
    const queue = this.queue
    queue.push(x)

    const size = queue.length
    for (let i = 1; i < size; i += 1) {
      queue.push(queue.shift())
    }
  }

  /**
   * @returns {number}
  */
  pop () {
    return this.queue.shift()
  }

  /**
   * @return {number}
  */
  top () {
    return this.queue[0]
  }

  /**
   * @return {boolean}
  */
  empty () {
    return this.queue.length < 1
  }
}

async function main () {
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
