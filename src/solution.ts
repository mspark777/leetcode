class PriorityQueue {
  readonly durations: number[]
  constructor () {
    this.durations = []
  }

  peek (): number {
    return this.durations.at(-1) as number
  }

  dequeue (): number {
    return this.durations.pop() as number
  }

  enqueue (duration: number) {
    let contain = false
    const durations = this.durations

    for (let i = 0; i < durations.length; i += 1) {
      if (durations[i] > duration) {
        durations.splice(i, 0, duration)
        contain = true
        break
      }
    }

    if (!contain) {
      durations.push(duration)
    }
  }

  isNotEmpty () {
    return this.getLength() > 0
  }

  getLength (): number {
    return this.durations.length
  }
}

export function scheduleCourse (courses: number[][]): number {
  courses.sort((a, b) => a[1] - b[1])
  const queue = new PriorityQueue()

  let time = 0
  for (const [duration, last] of courses) {
    const newTime = time + duration
    if (newTime <= last) {
      queue.enqueue(duration)
      time = newTime
    } else if (queue.isNotEmpty() && queue.peek() > duration) {
      time += duration - queue.dequeue()
      queue.enqueue(duration)
    }
  }

  return queue.getLength()
}
