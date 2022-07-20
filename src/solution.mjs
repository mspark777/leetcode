export function numMatchingSubseq (s, words) {
  const seen = new Map()
  let result = 0
  for (const word of words) {
    const check = seen.get(word)
    if (check === true) {
      result += 1
      continue
    } else if (check === false) {
      continue
    }

    let matched = 0
    for (let i = 0, j = 0; i < s.length && j < word.length; i += 1) {
      if (s.charCodeAt(i) === word.charCodeAt(j)) {
        matched += 1
        j += 1
      }
    }

    if (matched === word.length) {
      result += 1
      seen.set(word, true)
    } else {
      seen.set(word, false)
    }
  }

  return result
}
