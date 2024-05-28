# Product-shop

GraphQL backend for an imaginary Product shop implemented in Rust, `rocket`, `async-graphql` and `sqlx`. For detailed installation guide please refer to the [bootstrap project](https://github.com/lionkeng/sputnik). The only difference is that I used SQLx CLI instead of golang tool mentioned there.

# Project-specific implementation details

## Schema

GraphQL schema is quite simple:

```rust
enum ProductType {
    NORMAL,
    SPECIAL,
}

type Product {
    id: Int!
    name: String!
    description: String!
    price: Int!
    productType: ProductType!
    fillings: [String]
    images: [String]!
}
```

[Requests](/src/models/mod.rs) include querying, updating a price of a product and deleting it.

## Using SQLx CLI to perform migrations

To facilitate the migrations flow I recommend using the [SQLx CLI](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli). It allows you to add and run migrations in a clean fashion. Please refer to [migrations](/migrations) folder to see some examples.

cargo install sqlx-cli --no-default-features --features native-tls,postgres
sqlx migrate run

## Running the service

The current configuration in `Rocket.toml` has been setup to run the micro-service on port 7600. After you have `cargo build` and `cargo run`, you can play with the GraphQL at [localhost:7600](http://localhost:7600/)



