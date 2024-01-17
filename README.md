# Rust on Steroids

Rust Dev Machine ready to be used by you and your team with either Codespace or DevContainers.

## Built for Rust
- Uses Very Fast Linker [MOLD](https://github.com/rui314/mold) , set as default linker
- Uses [Sccache](https://github.com/mozilla/sccache) for caching crates
- Bonus Vscode Plugins for 10X Rust Developers
  - [cargo runner](https://marketplace.visualstudio.com/items?itemName=masterustacean.cargo-runner) - Run , Test , Doc-Test , Build Without Mental Overhead
  - [cargo bin](https://marketplace.visualstudio.com/items?itemName=masterustacean.cargo-bin) - Run any cargo bin commands on your $CARGO_HOME/bin

## Requirements
- Vscode
- Codespace
- Devcontainer

## Usage
- clone this repo 
- 
```sh
git clone https://github.com/codeitlikemiley/rust-on-steroids YOUR_NEXT_BIG_PROJECT
```

- update `.env.mysql` or `.env.postgres` depending on your db of choice, then `cp .env.example .env` on `.devcontainer` folder

e.g. change Database URL on `.devcontainer/.env` if you wanna use external service like planetscale, you can set it here.

```
DATABASE_URL=postgresql://postgres:secret@db:5432/db_name?sslmode=disable
```

- Update `.devcontainer/.bash_aliases` 

- Open Command Palette type `Open Folder in Container` or `Create new Codespace`

- Open Command Palette type `Open Terminal`

```sh
cd crates
```

## CLI Tools Bundled

- [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
- [cargo-chef](https://github.com/LukeMathWalker/cargo-chef)
- [cargo-generate](https://github.com/cargo-generate/cargo-generate)
- [cargo-hack](https://github.com/taiki-e/cargo-hack)
- [cargo-lambda](https://www.cargo-lambda.info/guide/getting-started.html)
- [cargo-leptos](https://book.leptos.dev/ssr/21_cargo_leptos.html)
- [cargo-nextest](https://nexte.st)
- [cargo-shuttle](https://docs.shuttle.rs/introduction/welcome)
- [cargo-watch](https://watchexec.github.io/docs/glob-patterns.html)
- [cloak](https://cloak.software/docs/getting-started/introduction/)
- [snip-cli](https://github.com/codeitlikemiley/snip-cli)
- [ws-cli](https://github.com/codeitlikemiley/ws-cli)
- [pulumi](https://www.pulumi.com/docs/)

## GPG Keys Issue?

- Export keys
```sh
gpg --list-secret-keys
gpg --armor --export-secret-keys [key-ID] > privatekey.asc
```

- Copy the key
```
cat privatekey.asc | pbcopy
```

- Create a new file on /workspace/private.asc and paste the key there

```sh
code /workspace/private.asc
```

- Import they key
```sh
gpg --import private.asc
```

## Git Config
- open .devcontainer/Dockerfile and uncomment

```yml
# Uncomment
# RUN git config --global user.email "codeitlikemiley@gmail.com"
# RUN  git config --global user.name "Uriah Galang"
```
