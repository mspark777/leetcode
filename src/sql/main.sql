select
    id,
    sum(case when month = 'Jan' then revenue else null end) as jan_revenue,
    sum(case when month = 'Feb' then revenue else null end) as feb_revenue,
    sum(case when month = 'Mar' then revenue else null end) as mar_revenue,
    sum(case when month = 'Apr' then revenue else null end) as apr_revenue,
    sum(case when month = 'May' then revenue else null end) as may_revenue,
    sum(case when month = 'Jun' then revenue else null end) as jun_revenue,
    sum(case when month = 'Jul' then revenue else null end) as jul_revenue,
    sum(case when month = 'Aug' then revenue else null end) as aug_revenue,
    sum(case when month = 'Sep' then revenue else null end) as sep_revenue,
    sum(case when month = 'Oct' then revenue else null end) as oct_revenue,
    sum(case when month = 'Nov' then revenue else null end) as nov_revenue,
    sum(case when month = 'Dec' then revenue else null end) as dec_revenue
from department
group by id
;

