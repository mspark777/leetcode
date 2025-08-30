import "@total-typescript/ts-reset";

function countBalls(lowLimit: number, highLimit: number): number {
  const counts = new Map<number, number>();

  for (let i = lowLimit; i <= highLimit; i += 1) {
    let sum = 0;
    let n = i;
    while (n > 0) {
      sum += n % 10;
      n = Number(BigInt(n) / 10n);
    }
    counts.set(sum, (counts.get(sum) ?? 0) + 1);
  }

  return Math.max(...counts.values());
}

interface Input {
  lowLimit: number;
  highLimit: number;
}

function main(): void {
  const inputs: Input[] = [
    {
      lowLimit: 1,
      highLimit: 10,
    },
    {
      lowLimit: 5,
      highLimit: 15,
    },
    {
      lowLimit: 19,
      highLimit: 28,
    },
  ];

  for (const input of inputs) {
    const result = countBalls(input.lowLimit, input.highLimit);
    console.log(result);
  }
}
main();
