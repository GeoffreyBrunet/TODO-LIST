# TODO-LIST
Todo List app in rust with Rocket &amp; Diesel

## Migrate database
I have used PostgreSQL in version 12, install on ubuntu on my laptop.  
Export database:
```shell
pg_dump -U postgres -h localhost -p 5432 -d todo-bdd > dbexport.pgsql
```

Import database:
```shell
pg_dump -U postgres -h localhost -p 5432 -d todo-bdd < dbexport.pgsql
```

## Install and run project

### Install rust:
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Set rust nightly version (in project folder):
```shell
rustup override set nightly
```


### Build and run API
```shell
cargo run
```

## Screenshots
In "img" folder, it's screenshots of my postman, who's running requests on my API.
