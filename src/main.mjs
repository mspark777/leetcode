import { createRequire } from 'module'
import { scheduleCourse } from './solution.mjs'

const require = createRequire(import.meta.url)

async function main () {
  const inputs = [
    { courses: [[100, 200], [200, 1300], [1000, 1250], [2000, 3200]] },
    { courses: [[1, 2]] },
    { courses: [[3, 2], [4, 3]] },
    require('./data.json')
  ]

  for (const input of inputs) {
    const result = scheduleCourse(input.courses)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
