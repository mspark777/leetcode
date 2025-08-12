select u.unique_id as unique_id, e.name as name
from employees as e
left outer join employeeuni as u on e.id = u.id
;

