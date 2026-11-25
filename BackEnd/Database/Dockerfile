FROM mysql:latest@sha256:9092489f49fb59c4ba6c1425c6a59f71f864f24bf4339a5a26b718adc7a8b343

# Set environment variables for MySQL setup
ENV MYSQL_DATABASE=masjidappdatabase

# Copy custom initialization scripts into the container
COPY ./01-create-database.sql /docker-entrypoint-initdb.d/
COPY ./02-create-tables.sql /docker-entrypoint-initdb.d/
COPY ./03-create-procedures.sql /docker-entrypoint-initdb.d/
COPY ./04-create-admin-users.sql /docker-entrypoint-initdb.d/
COPY ./05-create-public-users.sql /docker-entrypoint-initdb.d/
