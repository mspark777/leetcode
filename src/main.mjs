/**
 * @param {number[]} changed
 * @returns {number[]}
 */
function findOriginalArray (changed) {
  if ((changed.length % 2) === 1) {
    return []
  }

  const queue = []
  const result = []
  changed.sort((a, b) => a - b)

  let head = 0
  for (const i of changed) {
    if (queue[head] === i) {
      head += 1
    } else {
      result.push(i)
      queue.push(i * 2)
    }
  }

  return queue.length === head ? result : []
}

async function main () {
  const inputs = [
    {
      changed: [1, 3, 4, 2, 6, 8]
    },
    {
      changed: [6, 3, 0, 1]
    },
    {
      changed: [1]
    },
    {
      changed: [0, 0, 0, 0]
    }
  ]

  for (const { changed } of inputs) {
    const result = findOriginalArray(changed)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
