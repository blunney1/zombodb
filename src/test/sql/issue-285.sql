  select (zdb.highlight(ctid, 'payload.commits.message'))[1]
    from events
   where events ==> 'payload.commits.message:*beer*' and (zdb.highlight(ctid, 'payload.commits.message'))[1] ilike '%view%'
order by id limit 10;