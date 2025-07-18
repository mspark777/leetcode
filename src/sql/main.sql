select s.product_id as product_id, p.product_name as product_name
from sales as s
join product as p on s.product_id = p.product_id
group by s.product_id, p.product_name
having min(s.sale_date) >= '2019-01-01' and max(s.sale_date) <= '2019-03-31'
;

