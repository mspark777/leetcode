interface Sets {
  readonly made: Set<number>
  readonly todo: Set<number>
}

function movesToStamp (stamp: string, target: string): number[] {
  const stampLen = stamp.length
  const targetLen = target.length
  const queue: number[] = []
  const stack: number[] = []
  const list: Sets[] = []
  const dones = new Array<boolean>(targetLen).fill(false)

  for (let i = 0; i <= targetLen - stampLen; i += 1) {
    const made = new Set<number>()
    const todo = new Set<number>()

    for (let j = 0; j < stampLen; j += 1) {
      const k = i + j
      if (target.charCodeAt(k) === stamp.charCodeAt(j)) {
        made.add(k)
      } else {
        todo.add(k)
      }
    }

    list.push({ made, todo })
    if (todo.size < 1) {
      stack.push(i)
      for (let j = i; j < i + stampLen; j += 1) {
        if (!dones[j]) {
          queue.push(j)
          dones[j] = true
        }
      }
    }
  }

  while (queue.length > 0) {
    const i = queue.shift() as number
    for (let j = Math.max(0, i - stampLen + 1); j <= Math.min(targetLen - stampLen, i); j += 1) {
      const node = list[j]
      const todo = node.todo
      if (todo.has(i)) {
        todo.delete(i)
        if (todo.size < 1) {
          stack.push(j)
          for (const m of node.made) {
            if (!dones[m]) {
              queue.push(m)
              dones[m] = true
            }
          }
        }
      }
    }
  }

  for (const done of dones) {
    if (!done) {
      return []
    }
  }

  const result = new Array<number>(stack.length)
  for (let i = 0; stack.length > 0; i += 1) {
    result[i] = stack.pop() as number
  }

  return result
}

interface Input {
  readonly stamp: string
  readonly target: string
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      stamp: 'abc', target: 'ababc'
    },
    {
      stamp: 'abca', target: 'aabcaca'
    },
    {
      stamp: 'aye', target: 'eyeye'
    }
  ]

  for (const { target, stamp } of inputs) {
    const result = movesToStamp(stamp, target)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
