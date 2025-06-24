#[macro_export]
macro_rules! define_orm_with_common_fields {
    ($name:ident { $($field:ident : $typ:ty),* $(,)?}) => {
        #[derive(sqlx::FromRow, Debug)]
        pub struct $name {
            pub id: Option<i32>,
            pub uid: Option<uuid::Uuid>,
            pub created_at: Option<time::OffsetDateTime>,
            pub updated_at: Option<time::OffsetDateTime>,
            $(pub $field: $typ,)*
        }
    };
}

#[macro_export]
macro_rules! define_struct_with_common_fields {
    ($name:ident { $($field:ident : $typ:ty),* $(,)?}) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub id: Option<i32>,
            pub uid: Option<uuid::Uuid>,
            pub created_at: Option<time::OffsetDateTime>,
            pub updated_at: Option<time::OffsetDateTime>,
            $(pub $field: $typ,)*
        }
    };
}

#[macro_export]
macro_rules! define_to_with_common_fields {
    ($name:ident { $($field:ident : $typ:ty),* $(,)?}) => {
        #[derive(serde::Deserialize, serde::Serialize, Debug)]
        pub struct $name {
            pub id: Option<i32>,
            pub uid: Option<uuid::Uuid>,
            pub created_at: Option<time::OffsetDateTime>,
            pub updated_at: Option<time::OffsetDateTime>,
            $(pub $field: $typ,)*
        }
    };
}