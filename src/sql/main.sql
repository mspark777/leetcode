with user_count as (select count(user_id) as user_count from users)
select
    r.contest_id,
    round(
        ((count(r.contest_id)::decimal / uc.user_count::decimal) * 100)::decimal, 2
    ) as percentage
from register r
cross join user_count uc
group by r.contest_id, uc.user_count
order by percentage desc, r.contest_id asc

