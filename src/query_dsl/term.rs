//! This Module is to
//! https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-term-query.html
//!
//! Returns documents that contain an exact term in a provided field

mod dsl {
    use crate::zdbquery::ZDBQuery;
    use pgx::*;
    use serde::*;
    use serde_json::*;

    #[derive(Serialize)]
    struct Term<T> {
        value: T,
        #[serde(skip_serializing_if = "Option::is_none")]
        boost: Option<f32>,
    }

    /// ```funcname
    /// term
    /// ```
    #[pg_extern(immutable, parallel_safe)]
    pub(super) fn term_str(
        field: &str,
        value: &str,
        boost: Option<default!(f32, NULL)>,
    ) -> ZDBQuery {
        let term = Term { value, boost };
        make_term_dsl(field, term)
    }

    /// ```funcname
    /// term
    /// ```
    #[pg_extern(immutable, parallel_safe)]
    pub(super) fn term_bool(
        field: &str,
        value: bool,
        boost: Option<default!(f32, NULL)>,
    ) -> ZDBQuery {
        let term: Term<bool> = Term { value, boost };
        make_term_dsl(field, term)
    }

    /// ```funcname
    /// term
    /// ```
    #[pg_extern(immutable, parallel_safe)]
    pub(super) fn term_i16(
        field: &str,
        value: i16,
        boost: Option<default!(f32, NULL)>,
    ) -> ZDBQuery {
        let term: Term<i16> = Term { value, boost };
        make_term_dsl(field, term)
    }

    /// ```funcname
    /// term
    /// ```
    #[pg_extern(immutable, parallel_safe)]
    pub(super) fn term_i32(
        field: &str,
        value: i32,
        boost: Option<default!(f32, NULL)>,
    ) -> ZDBQuery {
        let term: Term<i32> = Term { value, boost };
        make_term_dsl(field, term)
    }

    /// ```funcname
    /// term
    /// ```
    #[pg_extern(immutable, parallel_safe)]
    pub(super) fn term_i64(
        field: &str,
        value: i64,
        boost: Option<default!(f32, NULL)>,
    ) -> ZDBQuery {
        let term: Term<i64> = Term { value, boost };
        make_term_dsl(field, term)
    }

    /// ```funcname
    /// term
    /// ```
    #[pg_extern(immutable, parallel_safe)]
    pub(super) fn term_f32(
        field: &str,
        value: f32,
        boost: Option<default!(f32, NULL)>,
    ) -> ZDBQuery {
        let term: Term<f32> = Term { value, boost };
        make_term_dsl(field, term)
    }

    /// ```funcname
    /// term
    /// ```
    #[pg_extern(immutable, parallel_safe)]
    pub(super) fn term_f64(
        field: &str,
        value: f64,
        boost: Option<default!(f32, NULL)>,
    ) -> ZDBQuery {
        let term: Term<f64> = Term { value, boost };
        make_term_dsl(field, term)
    }

    /// ```funcname
    /// term
    /// ```
    #[pg_extern(immutable, parallel_safe)]
    pub(super) fn term_time(
        field: &str,
        value: Time,
        boost: Option<default!(f32, NULL)>,
    ) -> ZDBQuery {
        let term: Term<Time> = Term { value, boost };
        make_term_dsl(field, term)
    }

    /// ```funcname
    /// term
    /// ```
    #[pg_extern(immutable, parallel_safe)]
    pub(super) fn term_date(
        field: &str,
        value: Date,
        boost: Option<default!(f32, NULL)>,
    ) -> ZDBQuery {
        let term: Term<Date> = Term { value, boost };
        make_term_dsl(field, term)
    }

    /// ```funcname
    /// term
    /// ```
    #[pg_extern(immutable, parallel_safe)]
    pub(super) fn term_time_with_timezone(
        field: &str,
        value: TimeWithTimeZone,
        boost: Option<default!(f32, NULL)>,
    ) -> ZDBQuery {
        let term: Term<TimeWithTimeZone> = Term { value, boost };
        make_term_dsl(field, term)
    }

    /// ```funcname
    /// term
    /// ```
    #[pg_extern(immutable, parallel_safe)]
    pub(super) fn term_timestamp(
        field: &str,
        value: Timestamp,
        boost: Option<default!(f32, NULL)>,
    ) -> ZDBQuery {
        let term: Term<Timestamp> = Term { value, boost };
        make_term_dsl(field, term)
    }

    /// ```funcname
    /// term
    /// ```
    #[pg_extern(immutable, parallel_safe)]
    pub(super) fn term_timestamp_with_timezone(
        field: &str,
        value: TimestampWithTimeZone,
        boost: Option<default!(f32, NULL)>,
    ) -> ZDBQuery {
        let term: Term<TimestampWithTimeZone> = Term { value, boost };
        make_term_dsl(field, term)
    }

    #[inline]
    fn make_term_dsl<T: serde::Serialize>(field: &str, term: Term<T>) -> ZDBQuery {
        ZDBQuery::new_with_query_dsl(json! {
            {
                "term": {
                    field: term,
                }
            }
        })
    }
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use crate::zdbquery::ZDBQuery;
    use pgx::*;
    use serde_json::json;
    use std::f32::{INFINITY, NAN, NEG_INFINITY};

    #[pg_test]
    fn test_term_str() {
        let boost = 42.0 as f32;
        let zdbquery =
            Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', 'test value','42.0');")
                .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "test value", "boost": boost}}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_str_with_default_boost() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', 'test value');")
            .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "test value"}}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_bool_true() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', true,'42.0');")
            .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname": {"value": true,"boost":42.0}}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_bool_false() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', false,'42.0');")
            .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname": {"value": false,"boost":42.0}}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_bool_true_with_default_boost() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', true);")
            .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname": {"value": true}}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_bool_false_with_default_boost() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', false);")
            .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname": {"value": false}}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_positive_i16() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', 32767, 42.0);")
            .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": 32767,"boost":42.0}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_negative_i16() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', -32700, 42.0);")
            .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": -32700,"boost":42.0}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_i16_with_default_boost() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', 32767);")
            .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": 32767}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_positive_i32() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', 2147483647, 42.0);")
            .expect("didn't get SPI return value");
        // let zdbquery = term_i32("fieldname", 2147483647, 42.0);
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": 2147483647,"boost":42.0}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_negative_i32() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', -2147483648, 42.0);")
            .expect("didn't get SPI return value");
        // let zdbquery = term_i32("fieldname", -2147483648, 42.0);
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": -2147483648,"boost":42.0}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_i32_with_default_boost() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', 2147483647);")
            .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": 2147483647}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_positive_i64() {
        let zdbquery =
            Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', 214740083647,'42.0');")
                .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();
        let value = 214740083647 as i64;

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": value,"boost":42.0}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_negative_i64() {
        let zdbquery =
            Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', -214740083647,'42.0');")
                .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();
        let value = -214740083647 as i64;

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": value, "boost": 42.0}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_i64_with_default_boost() {
        let value = 9223372036854775000 as i64;
        let zdbquery =
            Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', 9223372036854775000);")
                .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": value}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_positive_f32() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', 4.6, 42.0);")
            .expect("didn't get SPI return value");
        let value = 4.6;
        // let zdbquery = term_f32("fieldname", value, 42.0);
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": value,"boost": 42.0}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_negative_f32() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', -4.8, 42.0);")
            .expect("didn't get SPI return value");
        let value = -4.8;
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": value,"boost":42.0}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_f32_with_default_boost() {
        let value = 5.6;
        let zdbquery =
            Spi::get_one::<ZDBQuery>(&format!("SELECT dsl.term('fieldname', {});", value))
                .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": value}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_f32_with_positive_infinity() {
        let value = INFINITY;
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', 'infinity'::real);")
            .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": value}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_f32_with_negative_infinity() {
        let value = NEG_INFINITY;
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', '-infinity'::real);")
            .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": value}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_f32_with_nan() {
        let value = NAN;
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', 'nan'::real);")
            .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": value}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_positive_f64() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', 5.6, 42.0);")
            .expect("didn't get SPI return value");
        let value = 5.6;
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": value,"boost":42.0}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_negative_f64() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', -5.6, 42.0);")
            .expect("didn't get SPI return value");
        let value = -5.6;
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": value,"boost":42.0}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_f64_with_default_boost() {
        let value = 5.6;
        let zdbquery = Spi::get_one::<ZDBQuery>(&format!(
            "SELECT dsl.term('fieldname', {}::double precision);",
            value
        ))
        .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": value}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_f64_with_infinity() {
        let value = std::f64::INFINITY;
        let zdbquery =
            Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', 'infinity'::double precision);")
                .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": value}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_f64_with_negative_infinity() {
        let value = std::f64::NEG_INFINITY;
        let zdbquery = Spi::get_one::<ZDBQuery>(
            "SELECT dsl.term('fieldname', '-infinity'::double precision);",
        )
        .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": value}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_f64_with_nan() {
        let value = std::f64::NAN;
        let zdbquery =
            Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', 'nan'::double precision);")
                .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term":{"fieldname":{"value": value}}
                }
            }
        )
    }

    #[pg_test]
    fn test_term_date() {
        let zdbquery =
            Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', CAST('2020-01-01' AS date) );")
                .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "2020-01-01"}}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_date_with_boost() {
        let zdbquery = Spi::get_one::<ZDBQuery>(
            "SELECT dsl.term('fieldname', CAST('2020-01-01' AS date), 42.0 );",
        )
        .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "2020-01-01" , "boost": 42.0 as f32 }}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_time() {
        let zdbquery =
            Spi::get_one::<ZDBQuery>("SELECT dsl.term('fieldname', CAST('13:15:35' AS time) );")
                .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "13:15:35Z"}}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_time_with_boost() {
        let zdbquery = Spi::get_one::<ZDBQuery>(
            "SELECT dsl.term('fieldname', CAST('12:59:35' AS time), 42.0 );",
        )
        .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "12:59:35Z" , "boost": 42.0 as f32 }}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_time_with_milliseconds_and_with_boost() {
        let zdbquery = Spi::get_one::<ZDBQuery>(
            "SELECT dsl.term('fieldname', CAST('12:59:35.567' AS time), 42.0 );",
        )
        .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "12:59:35.567Z" , "boost": 42.0 as f32 }}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_time_with_timezone() {
        let zdbquery = Spi::get_one::<ZDBQuery>(
            "SELECT dsl.term('fieldname', CAST('13:15:35 +0900' AS time) );",
        )
        .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "13:15:35Z"}}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_time_with_boost_and_with_timezone() {
        let zdbquery = Spi::get_one::<ZDBQuery>(
            "SELECT dsl.term('fieldname', CAST('12:59:35 +0830' AS time), 42.0 );",
        )
        .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "12:59:35Z" , "boost": 42.0 as f32 }}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_time_with_milliseconds_with_timezone_and_with_boost() {
        let zdbquery = Spi::get_one::<ZDBQuery>(
            "SELECT dsl.term('fieldname', CAST('12:59:35.567 -1200' AS time), 42.0 );",
        )
        .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "12:59:35.567Z" , "boost": 42.0 as f32 }}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_timestamp() {
        let zdbquery = Spi::get_one::<ZDBQuery>(
            "SELECT dsl.term('fieldname', CAST('12-12-12 13:15:35' AS timestamp) );",
        )
        .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "2012-12-12T13:15:35Z"}}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_timestamp_with_boost() {
        let zdbquery = Spi::get_one::<ZDBQuery>(
            "SELECT dsl.term('fieldname', CAST('2013-04-10 12:59:35' AS timestamp), 42.0 );",
        )
        .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "2013-04-10T12:59:35Z" , "boost": 42.0 as f32 }}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_timestamp_with_milliseconds_and_with_boost() {
        let zdbquery = Spi::get_one::<ZDBQuery>(
            "SELECT dsl.term('fieldname', CAST('2019-09-15 12:59:35.567' AS timestamp), 42.0 );",
        )
        .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "2019-09-15T12:59:35.567Z" , "boost": 42.0 as f32 }}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_timestamp_with_timezone() {
        let zdbquery = Spi::get_one::<ZDBQuery>(
            "SELECT dsl.term('fieldname', CAST('12-12-12 13:15:35 -0700' AS timestamp) );",
        )
        .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "2012-12-12T13:15:35Z"}}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_timestamp_with_timezone_with_boost() {
        let zdbquery = Spi::get_one::<ZDBQuery>(
            "SELECT dsl.term('fieldname', CAST('2013-04-10 12:59:35 -0700' AS timestamp), 42.0 );",
        )
        .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "2013-04-10T12:59:35Z" , "boost": 42.0 as f32 }}
                }
            }
        );
    }

    #[pg_test]
    fn test_term_timestamp_with_timezone_with_milliseconds_and_with_boost() {
        let zdbquery = Spi::get_one::<ZDBQuery>(
            "SELECT dsl.term('fieldname', CAST('2019-09-15 12:59:35.567 -0700' AS timestamp), 42.0 );",
        )
        .expect("didn't get SPI return value");
        let dsl = zdbquery.query_dsl();

        assert!(dsl.is_some());
        assert_eq!(
            dsl.unwrap(),
            &json! {
                {
                    "term": {"fieldname": { "value": "2019-09-15T12:59:35.567Z" , "boost": 42.0 as f32 }}
                }
            }
        );
    }
}
