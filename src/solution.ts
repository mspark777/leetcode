export function reconstructQueue (people: number[][]): number[][] {
  const queue = people.map(value => [...value])

  queue.sort((a, b) => a[0] === b[0]
    ? a[1] - b[1]
    : b[0] - a[0]
  )

  const result: number[][] = []
  for (const person of queue) {
    result.splice(person[1], 0, person)
  }

  return result
}
