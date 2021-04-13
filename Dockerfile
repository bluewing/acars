# Container configuration
FROM rust:1.51
WORKDIR /var/www
COPY . .

# Install dependencies and compile acars.
RUN cargo install --path .
RUN cargo build --package acars --bin acars

# Make the entrypoint script executable, and executes `entrypoint.sh` on container start.
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
