select
    m.employee_id, m.name, count(*) as reports_count, round(avg(e.age)) as average_age
from employees e
join employees m on e.reports_to = m.employee_id
group by m.employee_id, m.name
order by m.employee_id
;

