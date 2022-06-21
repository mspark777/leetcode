/**
 * @param {number} k
 * @param {number} n
 * @return {number[][]}
 */
export const combinationSum3 = function (k, n) {
  if (n > 45) {
    return []
  } else if (n < k) {
    return []
  }

  const visitSet = new Set()
  const results = []
  const stack = [{ list: [1], sum: 1 }]
  while (stack.length > 0) {
    const top = stack.pop()
    if (top.list.length === k && top.sum === n) {
      const key = top.list.join('')
      if (!visitSet.has(key)) {
        results.push(top.list)
        visitSet.add(key)
      }
      continue
    } else if (top.list.length >= k) {
      continue
    }

    const next = top.list[top.list.length - 1] + 1
    for (let i = next; i < 10; i += 1) {
      stack.push({
        list: [...top.list, i],
        sum: top.sum + i
      })
    }

    if (next < 10) {
      stack.push({
        list: [next],
        sum: next
      })
    }
  }

  return results
}
