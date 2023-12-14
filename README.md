# WebAssembly chess app

## Installation

### Dependencies

The project relies on Rust with wasm-pack and SvelteKit with TypeScript on
npm.

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

### The app itself

```sh
git clone https://gitlab-stud.elka.pw.edu.pl/kpalucki/zpr-2023z
cd zpr-2023z
npm install # installs all the frontend dependencies
npm run build:all # compiles the wasm module and builds the project
npm preview # launches the app
```

### Installation through Docker

```shell
docker build -t wasm-chess-game .
docker run --detach --name wasm-chess-game --rm -p 4173:4173 wasm-chess-game
```
