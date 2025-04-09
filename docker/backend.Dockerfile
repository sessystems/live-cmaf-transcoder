FROM cmaf-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs |  bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY --from=cmaf-frontend /app/dist /app/backend/frontend/
COPY . .

RUN cargo build --release
