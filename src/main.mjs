/**
 * @param {string} stamp
 * @param {string} target
 * @return {number[]}
 */
function movesToStamp (stamp, target) {
  const stampLen = stamp.length
  const targetLen = target.length
  const queue = []
  const stack = []
  const list = []
  const dones = new Array(targetLen).fill(false)

  for (let i = 0; i <= targetLen - stampLen; i += 1) {
    const made = new Set()
    const todo = new Set()

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
    const i = queue.shift()
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

  const result = new Array(stack.length)
  for (let i = 0; stack.length > 0; i += 1) {
    result[i] = stack.pop()
  }

  return result
}

async function main () {
  const inputs = [
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
