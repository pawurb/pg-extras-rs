/* Queries with active exclusive locks. */

SELECT
  pg_stat_activity.pid,
  pg_class.relname,
  pg_locks.transactionid::text,
  pg_locks.granted,
  pg_locks.mode,
  pg_stat_activity.query AS query_snippet,
  age(now(),pg_stat_activity.query_start) AS "age",
  pg_stat_activity.application_name AS application
FROM pg_stat_activity,pg_locks left
OUTER JOIN pg_class
  ON (pg_locks.relation = pg_class.oid)
WHERE pg_stat_activity.query <> '<insufficient privilege>'
  AND pg_locks.pid = pg_stat_activity.pid
  AND pg_locks.mode IN ('ExclusiveLock', 'AccessExclusiveLock', 'RowExclusiveLock')
  AND pg_stat_activity.pid <> pg_backend_pid()
ORDER BY query_start
LIMIT 20;
