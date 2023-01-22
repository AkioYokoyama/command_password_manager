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
cargo build --release
```

## Usage
Display list of registered passwords.
```bash
./target/release/command_password_manager list
```

Register your password.
```bash
./target/release/command_password_manager add <key>

ex)
./target/release/command_password_manager add github
```

Delete a registered password.
```bash
./target/release/command_password_manager delete <key>

ex)
./target/release/command_password_manager delete github
```

Copy the password specified by key to the clipboard.
```bash
./target/release/command_password_manager copy <key>

ex)
./target/release/command_password_manager copy github
```

Remove all passwords
```bash
./target/release/command_password_manager flush
```
