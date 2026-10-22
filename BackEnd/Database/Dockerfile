FROM mysql:latest@sha256:f51c9c01c16b0eb5d81bb70ab7e7fcb3b96abc6a54114be3a94e8e569cae7815

# Set environment variables for MySQL setup
ENV MYSQL_DATABASE=masjidappdatabase

# Copy custom initialization scripts into the container
COPY ./01-create-database.sql /docker-entrypoint-initdb.d/
COPY ./02-create-tables.sql /docker-entrypoint-initdb.d/
COPY ./03-create-procedures.sql /docker-entrypoint-initdb.d/
COPY ./04-create-admin-users.sql /docker-entrypoint-initdb.d/
COPY ./05-create-public-users.sql /docker-entrypoint-initdb.d/
