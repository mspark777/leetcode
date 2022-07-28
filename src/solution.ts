export function isAnagram (s: string, t: string): boolean {
  if (s.length !== t.length) {
    return false
  }

  const counter = new Map<string, number>()
  for (const ch of s) {
    const count = counter.get(ch) ?? 0
    counter.set(ch, count + 1)
  }

  for (const ch of t) {
    const count = counter.get(ch)
    if (!count) {
      return false
    }

    if (count < 1) {
      return false
    } else if (count === 1) {
      counter.delete(ch)
    } else {
      counter.set(ch, count - 1)
    }
  }

  return counter.size < 1
}
