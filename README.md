# WebAssembly chess app

## Installation

### Dependencies

The project relies on Rust with wasm-pack and SvelteKit with TypeScript on
npm.

```sh
# Rust toolchain
curl https://sh.rustup.rs -sSf | sh

# Rust wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 

# nvm, npm and other node.js related stuff
wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash

# when using nvm you need to source the nvm variables in order to gain access
# to all the installed node and npm binaries
export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

nvm install 'v20.9.0'
```

### The app itself

```sh
git clone https://gitlab-stud.elka.pw.edu.pl/kpalucki/zpr-2023z
cd zpr-2023z
npm install # installs all the frontend dependencies
npm run build:all # compiles the wasm module and builds the project
npm preview # launches the app
```
