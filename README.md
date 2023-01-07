<h1>Command Password Manager</h1>
Password management with CUI.
Also, copy to clipboard.

## Install

Add sqlx commands.
```bash
cargo install sqlx-cli
```

Create database.
```bash
sqlx database create --database-url "sqlite:db/passwords.db"
```

Execute migration file.
```bash
sqlx migrate run  --database-url "sqlite:db/passwords.db"
```

Code is built.
```bash
cargo build
```

## Usage
Display list of registered sites.
```bash
cargo run -- list
```

Register your password.
```bash
cargo run -- add <key>

ex)
cargo run -- add github
```

Delete a registered password.
```bash
cargo run -- delete <key>

ex)
cargo run -- delete github
```

Copy one of the registered password.
```bash
cargo run -- copy <key>

ex)
cargo run -- copy github
```

Remove all passwords
```bash
cargo run -- flush
```
