select
    query_name,
    round(avg(rating * 1.0 / position), 2) as quality,
    round(
        avg(case when rating < 3 then 1 else 0 end) * 100.0, 2
    ) as poor_query_percentage
from queries
group by query_name
;

