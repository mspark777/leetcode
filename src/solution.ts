export function isPossible (target: number[]): boolean {
  const queue = new PriorityQueue()
  let sum = target.reduce((acc, cur) => {
    queue.enqueue(cur)
    return acc + cur
  }, 0)

  while (true) {
    let top = queue.dequeue() as number
    if (top === 1) {
      break
    }

    sum -= top

    if ((top <= sum) || (sum < 1)) {
      return false
    }

    top %= sum
    sum += top
    if (top > 0) {
      queue.enqueue(top)
    } else {
      queue.enqueue(sum)
    }
  }

  return true
}

class PriorityQueue {
  readonly nums: number[]
  constructor () {
    this.nums = []
  }

  peek (): number {
    return this.nums.at(-1) as number
  }

  dequeue (): number {
    return this.nums.pop() as number
  }

  enqueue (n: number): void {
    const nums = this.nums

    if (this.isEmpty() || (this.peek() < n)) {
      nums.push(n)
      return
    }

    let begin = 0
    let end = nums.length
    while (begin < end) {
      const pos = Math.trunc((end + begin) / 2)
      const num = nums[pos]
      if (num < n) {
        begin = pos + 1
      } else if (num > n) {
        end = pos
      } else {
        nums.splice(pos, 0, n)
        return
      }
    }

    nums.splice(begin, 0, n)
  }

  isEmpty (): boolean {
    return this.getLength() < 1
  }

  isNotEmpty (): boolean {
    return !this.isEmpty()
  }

  getLength (): number {
    return this.nums.length
  }
}
