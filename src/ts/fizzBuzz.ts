export function fizzBuzz (n: number): string[] {
  const answers: string[] = []
  for (let i = 1; i <= n; i += 1) {
    const mod3 = (i % 3) === 0
    const mod5 = (i % 5) === 0
    if (mod3 && mod5) {
      answers.push('FizzBuzz')
    } else if (mod3) {
      answers.push('Fizz')
    } else if (mod5) {
      answers.push('Buzz')
    } else {
      answers.push(i.toString())
    }
  }

  return answers
};
