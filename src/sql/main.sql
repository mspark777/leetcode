select coalesce(t1.employee_id, t2.employee_id) as employee_id
from employees t1
full join salaries t2 on t1.employee_id = t2.employee_id
where t1.name is null or t2.salary is null
order by employee_id

