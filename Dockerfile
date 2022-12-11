# Use the official Rust image as the base.
FROM rust:latest

# Install the required dependencies.
RUN apt-get update && apt-get install -y pkg-config libssl-dev apache2 apache2-utils 

# Create a new user called "app" to run the app with.
RUN useradd -m app

# Add the wasm build target
RUN rustup target add wasm32-unknown-unknown

# install trunk
RUN cargo install --locked trunk

# Copy the app's source code from the host machine to the container.
COPY dist /var/www/html/

RUN echo "ServerName 127.0.0.1" >> /etc/apache2/apache2.conf

# Expose the port that the app will run on.
EXPOSE 80

# Build and Run the app when the container starts.
CMD ["apache2ctl", "-D", "FOREGROUD"]