select product_name, sum(unit) as unit
from products as p
join orders as o on o.product_id = p.product_id
where order_date between '2020-02-01' and '2020-02-29'
group by p.product_name
having sum(unit) >= 100

