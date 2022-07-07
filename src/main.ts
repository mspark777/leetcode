import { isInterleave } from './solution'

interface Input {
  readonly s1: string
  readonly s2: string
  readonly s3: string
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      s1: 'aabcc',
      s2: 'dbbca',
      s3: 'aadbbcbcac'
    },
    {
      s1: 'aabcc',
      s2: 'dbbca',
      s3: 'aadbbbaccc'
    },
    {
      s1: '',
      s2: '',
      s3: ''
    }
  ]

  for (const input of inputs) {
    const result = isInterleave(input.s1, input.s2, input.s3)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
