use chrono::{DateTime, TimeZone, Utc};

#[macro_use]
extern crate timeless_partialeq;

#[test]
fn it_ignores_fields_ending_with_at() {
    #[derive(Debug, TimelessPartialEq)]
    pub struct Post {
        pub id: i64,
        pub content: String,
        pub author: i32,
        pub created_at: DateTime<Utc>,
    }

    assert_eq!(
        Post {
            id: 1,
            content: "test".to_string(),
            author: 1,
            created_at: Utc.timestamp_millis_opt(1715017040672).unwrap(),
        },
        Post {
            id: 1,
            content: "test".to_string(),
            author: 1,
            created_at: Utc::now(),
        }
    )
}

#[test]
fn it_fails_without_custom_derive_macro() {
    #[derive(Debug, PartialEq)]
    pub struct Post {
        pub id: i64,
        pub content: String,
        pub author: i32,
        pub created_at: DateTime<Utc>,
        pub updated_at: Option<DateTime<Utc>>,
    }
    assert_ne!(
        Post {
            id: 1,
            content: "test".to_string(),
            author: 1,
            created_at: Utc.timestamp_millis_opt(1715017040672).unwrap(),
            updated_at: Some(Utc.timestamp_millis_opt(1715017020672).unwrap()),
        },
        Post {
            id: 1,
            content: "test".to_string(),
            author: 1,
            created_at: Utc::now(),
            updated_at: Some(Utc::now()),
        }
    )
}

#[test]
fn it_checks_optional_timestamps_accordingly() {
    #[derive(Debug, TimelessPartialEq)]
    pub struct Post {
        pub id: i64,
        pub content: String,
        pub author: i32,
        pub created_at: DateTime<Utc>,
        pub updated_at: Option<DateTime<Utc>>,
    }

    assert_ne!(
        Post {
            id: 1,
            content: "test".to_string(),
            author: 1,
            created_at: Utc.timestamp_millis_opt(1715017040672).unwrap(),
            updated_at: Some(Utc.timestamp_millis_opt(1715017020672).unwrap()),
        },
        Post {
            id: 1,
            content: "test".to_string(),
            author: 1,
            created_at: Utc::now(),
            updated_at: None,
        }
    )
}

#[test]
fn it_works_with_user_attribute_input() {
    #[derive(Debug, TimelessPartialEq)]
    #[exclude_suffix(at, date)]
    pub struct Post {
        pub id: i64,
        pub content: String,
        pub author: i32,
        pub updated_at: Option<DateTime<Utc>>,
        pub creation_date: DateTime<Utc>,
    }

    assert_eq!(
        Post {
            id: 1,
            content: "test".to_string(),
            author: 1,
            creation_date: Utc.timestamp_millis_opt(1715017040672).unwrap(),
            updated_at: Some(Utc.timestamp_millis_opt(1715017020672).unwrap()),
        },
        Post {
            id: 1,
            content: "test".to_string(),
            author: 1,
            creation_date: Utc::now(),
            updated_at: Some(Utc::now()),
        }
    )
}
