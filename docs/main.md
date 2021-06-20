```shell
diesel setup
diesel migration generate create-example
# Write code in `migrations/up.sql` & `down.sql`
diesel migration run
diesel migration redo
diesel print-schema > src/schema.rs
```

https://transform.tools/json-to-rust-serde

[Allowing a type that's Queryable and Insertable to autoincrement its ID #1440](https://github.com/diesel-rs/diesel/issues/1440)