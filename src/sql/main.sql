select
    p.product_id,
    case
        when sum(u.units) is not null
        then round(sum(u.units * p.price)::numeric / sum(u.units)::numeric, 2)
        else 0
    end as average_price
from prices as p
left join
    unitssold as u
    on p.product_id = u.product_id
    and u.purchase_date between p.start_date and p.end_date
group by p.product_id
;

