function validUtf8 (data: number[]): boolean {
  const mask1 = 128n
  const mask2 = 64n

  let bytes = 0n
  for (const n of data) {
    const i = BigInt(n)
    if (bytes === 0n) {
      let mask = 128n
      while ((mask & i) !== 0n) {
        bytes += 1n
        mask >>= 1n
      }

      if (bytes === 0n) {
        continue
      }

      if ((bytes > 4n) || (bytes === 1n)) {
        return false
      }
    } else {
      const check0 = i & mask1
      const check1 = i & mask2
      if ((check0 === 0n) || (check1 !== 0n)) {
        return false
      }
    }

    bytes -= 1n
  }

  return bytes === 0n
}

interface Input {
  readonly data: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      data: [197, 130, 1]
    },
    {
      data: [235, 140, 4]
    }
  ]

  for (const { data } of inputs) {
    const result = validUtf8(data)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
