import "@total-typescript/ts-reset";

function average(salary: number[]): number {
  const max = salary.reduce((a, b) => Math.max(a, b));
  const min = salary.reduce((a, b) => Math.min(a, b));
  const sum = salary.reduce((a, b) => a + b) - (max + min);
  const count = salary.length - 2;

  return Number((sum / count).toFixed(5));
}

interface Input {
  salary: number[];
}

function main(): void {
  const inputs: Input[] = [
    {
      salary: [4000, 3000, 1000, 2000],
    },
    {
      salary: [1000, 2000, 3000],
    },
  ];

  for (const input of inputs) {
    const result = average(input.salary);
    console.log(result);
  }
}
main();
