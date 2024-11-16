# Generate the `entities` module

To make changes to the database tables and schema, you will have to modify files in the `migrator` module and then update the `entities` module using `sea-orm-cli`.

This assumes that you already have the `sqlite3` binary installed. First, make sure you have installed `sea-orm-cli`:

```sh
$ cargo install sea-orm-cli
```

Make sure your working directory is in the crate root:

```sh
$ cd optd-persistent
```

If you have not generate the `sqlite.db` file yet, you will need to run this command which will generate the `sqlite.db` file and run all of the migrations:

```sh
$ cargo run --bin migrate
```

Finally, run this command to generate / overwrite the `entities` module in the `src` directory.

```
$ sea-orm-cli generate entity -u sqlite:./sqlite.db -o src/entities
```

Detailed ORM documentation resides in the `migrator` module instead of the generated `entities` module. To view as rustdoc, run

```sh
$ cargo doc --document-private-items --open
```

# Demo

To run the demo, run the root binary crate:

```sh
$ cargo run --bin optd-persistent
```
