import "@total-typescript/ts-reset";

class Employee {
  id: number;
  importance: number;
  subordinates: number[];
  constructor(id: number, importance: number, subordinates: number[]) {
    this.id = id;
    this.importance = importance;
    this.subordinates = subordinates;
  }
}

function dfs(id: number, map: Map<number, Employee>): number {
  const employee = map.get(id);
  if (employee == null) {
    throw new Error();
  }

  let importance = employee.importance;
  for (const subId of employee.subordinates) {
    importance += dfs(subId, map);
  }

  return importance;
}

function getImportance(employees: Employee[], id: number): number {
  const employeeMap = new Map<number, Employee>();
  for (const employee of employees) {
    employeeMap.set(employee.id, employee);
  }

  return dfs(id, employeeMap);
}

interface Input {
  nums: number[];
}

function main(): void {
  const inputs: Input[] = [
    {
      nums: [1, 3, 6, 10, 12, 15],
    },
    {
      nums: [1, 2, 4, 7, 10],
    },
    {
      nums: [4, 4, 9, 10],
    },
  ];

  for (const input of inputs) {
    const result = averageValue(input.nums);
    console.log(result);
  }
}
main();
