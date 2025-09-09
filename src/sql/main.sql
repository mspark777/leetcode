select user_id, max(time_stamp) last_stamp
from logins
where time_stamp::date between '2020-01-01' and '2020-12-31'
group by user_id

