#[macro_export]
macro_rules! common_fields {
    () => {
        pub id: Option<u16>,
        pub uid: Option<uuid::Uuid>,
        pub created_at: <time::OffsetDateTime>,
        pub updated_at: Option<time::OffsetDateTime>,
    };
}