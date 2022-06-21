/**
 * @param {string} s
 * @param {number} k
 * @return {string}
 */
export const removeDuplicates = function (s, k) {
  if (s.length < 2) {
    return s
  }

  let topPos = 0
  const stack = []
  for (let i = 0; i < s.length; i += 1) {
    const char = s[i]
    if (stack.length < 1) {
      topPos = 0
      stack.push({
        char,
        count: 1
      })
      continue
    }

    const top = stack[topPos]
    if (char === top.char) {
      top.count += 1
    } else {
      stack.push({
        char,
        count: 1
      })
      topPos += 1
    }

    if (stack[topPos].count === k) {
      stack.pop()
      topPos -= 1
    }
  }

  const result = []
  for (const node of stack) {
    for (let i = 0; i < node.count; i += 1) {
      result.push(node.char)
    }
  }

  return result.join('')
}
