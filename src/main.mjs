/**
  * @param {string} s
  * @param {string} goal
  * @returns {boolean}
  */
function buddyStrings (s, goal) {
  if (s.length !== goal.length) {
    return false
  }

  if (s === goal) {
    /** @type {Map<string, number>} */
    const counts = new Map()
    for (const c of s) {
      const count = counts.get(c) ?? 0
      if (count === 1) {
        return true
      } else {
        counts.set(c, count + 1)
      }
    }
  }

  let first = -1
  let second = -1
  for (let i = 0; i < s.length; i += 1) {
    const c = s.charAt(i)
    const g = goal.charAt(i)
    if (c === g) {
      continue
    }

    if (first < 0) {
      first = i
    } else if (second < 0) {
      second = i
    } else {
      return false
    }
  }

  if (first < 0) {
    return false
  } else if (second < 0) {
    return false
  }

  if (s.charAt(first) !== goal.charAt(second)) {
    return false
  } else if (s.charAt(second) !== goal.charAt(first)) {
    return false
  }

  return true
}

function main () {
  const inputs = [
    ['ab', 'ba'],
    ['ab', 'ab'],
    ['aa', 'aa']
  ]

  for (const [s, goal] of inputs) {
    const result = buddyStrings(s, goal)
    console.log(result)
  }
}
main()
