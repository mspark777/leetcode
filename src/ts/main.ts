import "@total-typescript/ts-reset";

function reformatDate(date: string): string {
  const [day, month, year] = date.split(" ") as [string, string, string];
  const monthMap = new Map(
    Object.entries({
      Jan: "01",
      Feb: "02",
      Mar: "03",
      Apr: "04",
      May: "05",
      Jun: "06",
      Jul: "07",
      Aug: "08",
      Sep: "09",
      Oct: "10",
      Nov: "11",
      Dec: "12",
    }),
  );
  return `${year}-${monthMap.get(month) as string}-${day.slice(0, -2).padStart(2, "0")}`;
}

interface Input {
  date: string;
}

function main(): void {
  const inputs: Input[] = [
    {
      date: "20th Oct 2052",
    },
    {
      date: "6th Jun 1933",
    },
    {
      date: "26th May 1960",
    },
  ];

  for (const input of inputs) {
    const result = reformatDate(input.date);
    console.log(result);
  }
}
main();
