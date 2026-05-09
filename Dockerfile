# =========================
# Stage 1: Build the app
# =========================
FROM rust:1.88 AS builder

# Install dependencies
RUN rustup target add wasm32-unknown-unknown

RUN cargo install trunk
RUN cargo install wasm-bindgen-cli

WORKDIR /app

# ---------------------------------
# Copy dependency files FIRST
# ---------------------------------
COPY Cargo.toml Cargo.lock ./

# Create dummy src so cargo can build deps
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

# Cache dependencies
RUN cargo build --release || true

# Remove dummy source
RUN rm -rf src

# ---------------------------------
# Copy actual project files
# ---------------------------------
COPY . .

# Build the actual app
RUN trunk build --release

# =========================
# Stage 2: Serve with nginx
# =========================
FROM nginx:alpine

# Remove default nginx files
RUN rm -rf /usr/share/nginx/html/*

# Copy built frontend from step 1
COPY --from=builder /app/dist /usr/share/nginx/html

COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]