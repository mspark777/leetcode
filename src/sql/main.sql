select
    sell_date,
    count(distinct(product, sell_date)) as num_sold,
    string_agg(distinct product, ',' order by product) as products
from activities
group by sell_date
order by sell_date
;

