use pgx::{
    elog, pg_sys, register_xact_callback, IntoDatum, PgOid, PgRelation, PgXactCallbackEvent, Spi,
    SpiTupleTable,
};

use crate::elasticsearch::Elasticsearch;
use crate::gucs::ZDB_LOG_LEVEL;
use crate::utils::{is_zdb_index, lookup_zdb_extension_oid};

pub fn drop_index(index: &PgRelation) {
    // we can only delete the remote index for actual ZDB indices
    if is_zdb_index(index) {
        // when the transaction commits, we'll make a best effort to delete this index
        // from its remote Elasticsearch server
        let es = Elasticsearch::new(index);
        register_xact_callback(PgXactCallbackEvent::Commit, move || {
            elog(
                ZDB_LOG_LEVEL.get().log_level(),
                &format!("[zombodb] Deleting remote index: {}", es.base_url()),
            );

            // we're just going to assume it worked, throwing away any error
            // because raising an elog(ERROR) here would cause Postgres to panic
            es.delete_index().execute().ok();
        });
    }
}

pub fn drop_table(table: &PgRelation) {
    for index in table.indicies().iter_oid() {
        let index = PgRelation::with_lock(index, pg_sys::AccessExclusiveLock as pg_sys::LOCKMODE);
        drop_index(&index);
    }
}

pub fn drop_schema(schema_oid: pg_sys::Oid) {
    Spi::connect(|client| {
        let table = client.select(
            "select oid from pg_class
                    where relnamespace = $1 
                      and relam = (select oid from pg_am where amname = 'zombodb')",
            None,
            Some(vec![(PgOid::from(pg_sys::OIDOID), schema_oid.into_datum())]),
        );
        drop_index_oids(table);
        Ok(Some(()))
    });
}

pub fn drop_extension(extension_oid: pg_sys::Oid) {
    if extension_oid == lookup_zdb_extension_oid() {
        Spi::connect(|client| {
            let table = client.select(
                "select oid from pg_class
                    where relam = (select oid from pg_am where amname = 'zombodb')",
                None,
                None,
            );
            drop_index_oids(table);
            Ok(Some(()))
        });
    }
}

fn drop_index_oids(mut table: SpiTupleTable) {
    while table.next().is_some() {
        let oid = table
            .get_one::<pg_sys::Oid>()
            .expect("returned index oid is NULL");
        let index = PgRelation::with_lock(oid, pg_sys::AccessExclusiveLock as pg_sys::LOCKMODE);
        drop_index(&index);
    }
}
