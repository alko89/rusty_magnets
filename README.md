# Rusty (diesel rocket) magnets

This is a small website for browsing magnet links from the internet written in Rust.
The work started in https://github.com/Alko89/todo. This is a cleaned up project set up
for further development.

## Todo

  1. Write web crawlers in Rust.

  2. Pagination on the GUI is not the best yet.

  3. Move database to MySQL or Postgre.


## Running

**Before running, building, or testing, you'll need to ensure the following:**

  1. A SQLite database file with the proper schema is present.

    ```
    cargo install diesel_cli                      # install diesel CLI tools
    mkdir db                                      # create db folder
    DATABASE_URL=db/db.sql diesel migration run   # create db/db.sql
    ```

  2. Export `DATABASE_URL` environment variable that points to the SQLite
     database file.

     * `export DATABASE_URL=db/db.sql`

  3. Run crawler and server.

     * `cargo run --bin crawler`
     * `cargo run --bin server`
