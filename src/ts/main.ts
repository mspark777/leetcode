import "@total-typescript/ts-reset";

function calcIdx(day: number, month: number, year: number): number {
  const d = BigInt(day);
  let m = BigInt(month);
  let y = BigInt(year);
  if (m < 3n) {
    m += 12n;
    y -= 1n;
  }

  const k = y % 100n;
  const j = y / 100n;

  const i = (d + (13n * (m + 1n)) / 5n + k + k / 4n + j / 4n + 5n * j) % 7n;
  return Number(i);
}

function dayOfTheWeek(day: number, month: number, year: number): string {
  const date = [
    "Saturday",
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
  ];

  const idx = calcIdx(day, month, year);
  return date[idx] as string;
}

interface Input {
  day: number;
  month: number;
  year: number;
}

function main(): void {
  const inputs: Input[] = [
    {
      day: 31,
      month: 8,
      year: 2019,
    },
    {
      day: 18,
      month: 7,
      year: 1999,
    },
    {
      day: 15,
      month: 8,
      year: 1993,
    },
  ];

  for (const input of inputs) {
    const result = dayOfTheWeek(input.day, input.month, input.year);
    console.log(result);
  }
}
main();
