function canVisitAllRooms (rooms: number[][]): boolean {
  const seen = new Array<boolean>(rooms.length).fill(false)
  seen[0] = true

  const stack = new Array<number>()
  stack.push(0)

  for (let top = stack.pop(); top != null; top = stack.pop()) {
    for (const key of rooms[top]) {
      if (!seen[key]) {
        seen[key] = true
        stack.push(key)
      }
    }
  }

  return seen.every(s => s)
}

async function main (): Promise<void> {
  const inputs: number[][][] = [
    [[1], [2], [3], []],
    [[1, 3], [3, 0, 1], [2], [0]]
  ]

  for (const rooms of inputs) {
    const result = canVisitAllRooms(rooms)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
