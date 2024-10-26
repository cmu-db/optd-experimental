# optd-experimental

Extensible SQL Query Optimizer Service

To run the migration I just called:

```
cargo run --manifest-path ./migration/Cargo.toml -- refresh -u "sqlite:///absolute/path/to/memory.db"

```

Notice the three forward slashes (///) after sqlite:. This is the correct format for a local SQLite database file.

Then run the following command to generate the entity files:

```
# In case you have not installed `sea-orm-cli`
cargo install sea-orm-cli

sea-orm-cli generate entity \
    -u sqlite:///Users/sarveshtandon/Development/optd-experimental/optd-persistent/memory.db \
    -o src/entities
```
