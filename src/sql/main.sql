SELECT MAX(salary) as SecondHighestSalary FROM Employee WHERE salary NOT IN (
  SELECT MAX(salary) FROM Employee
)
