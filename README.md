# To run the server:

## Clone the repo:
```
git clone https://github.com/shaggysa/leptos-prototyping.git
cd leptos-prototyping
```

## Install postgres, and create a database for the server:
```
sudo -iu postgres psql
```

## Inside the postgres terminal:
```
CREATE ROLE username WITH LOGIN PASSWORD 'password';

CREATE DATABASE databasename WITH OWNER username;

\q
```

## For the server to recognize the database, you need to put the info in a .env file:

```
touch .env

echo "DATABASE_URL=postgres://username:password@localhost:5432/databasename" >> .env
```

Finally, start the server:
```
cargo leptos watch
```


## If you do not have postgres installed already:

### macos:
```
brew install postgresql@15
```

### debian:
```
sudo apt install postgresql
```

### fedora:
```
sudo dnf install postgresql-server postgresql
```

### arch:
```
sudo pacman -S postgresql
```


## If you do not have cargo-leptos already:
```
cargo install --locked cargo-leptos
```
