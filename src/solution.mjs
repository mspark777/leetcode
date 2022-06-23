class PriorityQueue {
  constructor () {
    this.durations = []
  }

  peek () {
    return this.durations.at(-1)
  }

  dequeue () {
    return this.durations.pop()
  }

  enqueue (duration) {
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

  getLength () {
    return this.durations.length
  }
}

export function scheduleCourse (courses) {
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
