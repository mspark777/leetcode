/**
  * @param {string[]} ideas
  * @returns {number}
  */
function distinctNames (ideas) {
  /** @type {Map<string, Set<string>>} */
  const groupMap = new Map()

  for (const idea of ideas) {
    const first = idea.charAt(0)
    const remains = idea.substring(1)
    const group = groupMap.get(first) ?? new Set()
    group.add(remains)
    groupMap.set(first, group)
  }

  let result = 0
  const groups = [...groupMap.values()]
  for (let i = 0; i < groups.length - 1; i += 1) {
    const curgroup = groups[i]
    for (let j = i + 1; j < groups.length; j += 1) {
      const group = groups[j]
      let num = 0

      for (const idea of curgroup) {
        if (group.has(idea)) {
          num += 1
        }
      }

      result += 2 * (curgroup.size - num) * (group.size - num)
    }
  }

  return result
}

async function main () {
  const inputs = [
    ['coffee', 'donuts', 'time', 'toffee'],
    ['lack', 'back']
  ]

  for (const ideas of inputs) {
    const result = distinctNames(ideas)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
