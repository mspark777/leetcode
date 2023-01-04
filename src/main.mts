function minimumRounds(tasks: number[]): number {
  const frequencies = new Map<number, number>()
  for (const task of tasks) {
    const count = frequencies.get(task) ?? 0
    frequencies.set(task, count + 1)
  }

  let result = 0
  for (const count of frequencies.values()) {
    if (count === 1) {
      return -1
    }

    result += Math.trunc(count / 3)
    if ((count % 3) !== 0) {
      result += 1
    }
  }

  return result
}

async function main(): Promise<void> {
  const inputs: number[][] = [
    [2, 2, 3, 3, 2, 4, 4, 4, 4, 4],
    [2, 3, 3]
  ]

  for (const tasks of inputs) {
    const result = minimumRounds(tasks)
    console.log(result)
  }
}

await main()
