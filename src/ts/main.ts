import "@total-typescript/ts-reset";

function countTriples(n: number): number {
  let result = 0;

  for (let a = 1; a <= n; a += 1) {
    for (let b = 1; b <= n; b += 1) {
      const cSquared = a * a + b * b;
      const c = Math.trunc(Math.sqrt(cSquared));
      if (c * c === cSquared && c <= n) {
        result += 1;
      }
    }
  }

  return result;
}

interface Input {
  n: number;
}

function main(): void {
  const inputs: Input[] = [
    {
      n: 5,
    },
    {
      n: 10,
    },
  ];

  for (const input of inputs) {
    const result = countTriples(input.n);
    console.log(result);
  }
}
main();
