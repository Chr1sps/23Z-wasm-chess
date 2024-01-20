# WebAssembly chess app

## Installation

### Dependencies

- Rust toolchain (`rustup`) with `wasm-pack`
- node.js version 20 (Iron LTS) with npm

### Linux

```sh
# Rust toolchain
curl https://sh.rustup.rs -sSf | sh -s

# Rust wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# nvm, npm and other node.js related stuff
wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash

# when using nvm you need to source the nvm variables in order to add the npm
# and node.js binaries to the PATH
export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

nvm install 'v20.9.0'
npm install -g 'npm@10.2.5'
```

#### The app itself

```sh
git clone https://gitlab-stud.elka.pw.edu.pl/kpalucki/zpr-2023z
cd zpr-2023z
npm install # installs all the frontend dependencies
npm run build:all # compiles the wasm module and builds the project
npm preview # launches the app
```

### Docker

```shell
docker build -t wasm-chess-game .
docker run --detach --name wasm-chess-game --rm -p 4173:4173 wasm-chess-game
```

## Tests

The project utilizes Vitest for testing the frontend and cargo test for testing
the backend.

To run tests for both the frontend and the backend you can use a single
command:

```shell
npm run test:all
```

To run tests for each of them separately:

```shell
npm run test # runs vitest
npm run test:wasm # runs cargo test
```

## Linting and formatting

The frontend part utilizes Prettier with ESLint for formatting and linting,
while the WASM backend uses cargo's fmt and clippy.

Additionally, running

```shell
npm run check
```

will run TypeScript's typechecking on the frontend.
