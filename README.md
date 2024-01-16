# Rust on Steroids

Rust Dev Machine ready to be used by you and your team with either Codespace or DevContainers.

## Built for Rust
- Uses Very Fast Linker [MOLD](https://github.com/rui314/mold) , set as default linker
- Uses [Sccache] for caching crates
- Bonus Vscode Plugins for 10X Rust Developers
  - [cargo runner](https://marketplace.visualstudio.com/items?itemName=masterustacean.cargo-runner) - Run , Test , Doc-Test , Build Without Mental Overhead
  - [cargo bin](https://marketplace.visualstudio.com/items?itemName=masterustacean.cargo-bin) - Run any cargo bin commands on your $CARGO_HOME/bin

## Requirements
- Vscode
- Codespace
- Devcontainer

## Usage
- update `.env.mysql` or `.env.postgres` depending on your db of choice, then `cp .env.example .env` on `.devcontainer` folder

e.g. change Database URL on `.devcontainer/.env` if you wanna use external service like planetscale, you can set it here.

```
DATABASE_URL=postgresql://postgres:secret@db:5432/db_name?sslmode=disable
```

- Update `.devcontainer/.bash_aliases` 

- Open Command Palette type `Open Folder in Container` or `Create new Codespace`

## CLI Tools Bundled

- sqlx-cli
- cargo-chef
- cargo-generate
- cargo-hack
- cargo-lambda
- cargo-leptos
- cargo-nextest
- cargo-shuttle
- cargo-watch