# Rusty (diesel rocket) magnets

This is a small website for browsing magnet links from the internet written in Rust.
The work started in https://github.com/Alko89/todo. This is a cleaned up project set up
for further development.

## Todo

  1. Write web crawlers in Rust.

  2. Pagination on the GUI is not the best yet.

  3. Move database to MySQL or Postgre.


## Running

**Before running, building, or testing this example, you'll need to ensure the
following:**

  1. A SQLite database file with the proper schema is present.

    On a Unix machine or with bash installed, you can simply run the
    `boostrap.sh` script to create the database. The script installs the
    `diesel_cli` tools if they're not already installed and runs the migrations.
    The script will output a `DATABASE_URL` variable.

    You can also run the migrations manually with the following commands:

    ```
    cargo install diesel_cli                      # install diesel CLI tools
    mkdir db                                      # create db folder
    DATABASE_URL=db/db.sql diesel migration run   # create db/db.sql
    ```

  2. A `DATABASE_URL` environment variable is set that points to the SQLite
     database file.

     Use the `DATABASE_URL` variable emitted from the `bootstrap.sh` script, or
     enter it manually, as follows:

     * `DATABASE_URL=db/db.sql cargo build`
     * `DATABASE_URL=db/db.sql cargo test`
     * `DATABASE_URL=db/db.sql cargo run`
