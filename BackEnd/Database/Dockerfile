FROM mysql:latest@sha256:3819b2b23835c4a73c8b022fafc33c8095c0f81c929a8b7a84613d2fba6c52f5

# Set environment variables for MySQL setup
ENV MYSQL_DATABASE=masjidappdatabase

# Copy custom initialization scripts into the container
COPY ./01-create-database.sql /docker-entrypoint-initdb.d/
COPY ./02-create-tables.sql /docker-entrypoint-initdb.d/
COPY ./03-create-procedures.sql /docker-entrypoint-initdb.d/
COPY ./04-create-admin-users.sql /docker-entrypoint-initdb.d/
COPY ./05-create-public-users.sql /docker-entrypoint-initdb.d/
