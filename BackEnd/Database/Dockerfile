FROM mysql:latest

# Set environment variables for MySQL setup
ENV MYSQL_DATABASE=masjidappdatabase

# Copy custom initialization scripts into the container
COPY ./0*.sql /docker-entrypoint-initdb.d/
