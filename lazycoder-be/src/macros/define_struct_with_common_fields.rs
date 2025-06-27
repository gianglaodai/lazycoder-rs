#[macro_export]
macro_rules! define_orm_with_common_fields {
    ($name:ident { $($field:tt)* }) => {
        #[derive(sqlx::FromRow, Debug)]
        pub struct $name {
            pub id: Option<i32>,
            pub uid: Option<uuid::Uuid>,
            pub created_at: Option<time::OffsetDateTime>,
            pub updated_at: Option<time::OffsetDateTime>,
            $($field)*
        }
    };
}

#[macro_export]
macro_rules! define_struct_with_common_fields {
    ($name:ident { $($field:tt)* }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub id: Option<i32>,
            pub uid: Option<uuid::Uuid>,
            pub created_at: Option<time::OffsetDateTime>,
            pub updated_at: Option<time::OffsetDateTime>,
            $($field)*
        }
    };
}

#[macro_export]
macro_rules! define_to_with_common_fields {
    ($name:ident { $($field:tt)* }) => {
        #[derive(serde::Deserialize, serde::Serialize, Debug)]
        pub struct $name {
            #[serde(default)]
            pub id: Option<i32>,
            #[serde(default)]
            pub uid: Option<uuid::Uuid>,
            #[serde(default, with = "time::serde::timestamp::option")]
            pub created_at: Option<time::OffsetDateTime>,
            #[serde(default, with = "time::serde::timestamp::option")]
            pub updated_at: Option<time::OffsetDateTime>,
            $($field)*
        }
    };
}