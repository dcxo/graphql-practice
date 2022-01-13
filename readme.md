A little practice of how to create a GraphQL server using Rust Language and SQLite as database.

To run it:
1. Install the sqlx-cli
```shell
cargo install sqlx-cli
```
2. Create the database
```shell
cargo sqlx db create
```
3. Run the migrations
```shell
cargo sqlx mig run
```
4. Finally, run the application
```shell
cargo run
```