FROM mysql:latest@sha256:893418c8a7a34c60d2d961cc30ef136095da141478562ed2101ae3f5faba9c3f

# Set environment variables for MySQL setup
ENV MYSQL_DATABASE=masjidappdatabase

# Copy custom initialization scripts into the container
COPY ./01-create-database.sql /docker-entrypoint-initdb.d/
COPY ./02-create-tables.sql /docker-entrypoint-initdb.d/
COPY ./03-create-procedures.sql /docker-entrypoint-initdb.d/
COPY ./04-create-admin-users.sql /docker-entrypoint-initdb.d/
COPY ./05-create-public-users.sql /docker-entrypoint-initdb.d/
