FROM node:20-alpine
WORKDIR /app
COPY . .
RUN apk add curl build-base
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN npm install
RUN npm run build:all
CMD ["npm", "run", "host"]
