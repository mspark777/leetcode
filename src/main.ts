function readBinaryWatch (turnedOn: number): string[] {
  const result: string[] = []

  for (let h = 0; h < 12; h += 1) {
    for (let m = 0; m < 60; m += 1) {
      const num = (h << 6) | m
      const ones = num.toString(2).split('').filter(s => s === '1').length
      if (ones === turnedOn) {
        const time = `${h}${m < 10 ? ':0' : ':'}${m}`
        result.push(time)
      }
    }
  }

  return result
}

async function main (): Promise<void> {
  const inputs: number[] = [1, 9]

  for (const turnedOn of inputs) {
    const result = readBinaryWatch(turnedOn)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
