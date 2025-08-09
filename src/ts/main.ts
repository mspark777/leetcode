import "@total-typescript/ts-reset";

function daysBetweenDates(date1: string, date2: string): number {
  const d1 = new Date(date1);
  const d2 = new Date(date2);
  const diff = Math.abs(d1.getTime() - d2.getTime());
  return Number(BigInt(diff) / 86400000n);
}

interface Input {
  date1: string;
  date2: string;
}

function main(): void {
  const inputs: Input[] = [
    {
      date1: "2019-06-29",
      date2: "2019-06-30",
    },
    {
      date1: "2020-01-15",
      date2: "2019-12-31",
    },
  ];

  for (const input of inputs) {
    const result = daysBetweenDates(input.date1, input.date2);
    console.log(result);
  }
}
main();
