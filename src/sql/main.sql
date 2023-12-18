CREATE OR REPLACE FUNCTION NthHighestSalary(N INT) RETURNS TABLE (Salary INT) AS $$
BEGIN
  RETURN QUERY (
    -- Write your PostgreSQL query statement below.
    SELECT t1.salary from
    (SELECT e.salary, dense_rank() over(order by e.salary desc) as rank from Employee e) t1
    WHERE rank = N
    LIMIT 1
  );
END;
$$ LANGUAGE plpgsql;
