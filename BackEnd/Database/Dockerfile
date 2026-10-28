FROM mysql:latest@sha256:7b16ec7caa94ca7e92f4c5ec287de69d2e8738db056ceb629022951a6f6d910b

# Set environment variables for MySQL setup
ENV MYSQL_DATABASE=masjidappdatabase

# Copy custom initialization scripts into the container
COPY ./01-create-database.sql /docker-entrypoint-initdb.d/
COPY ./02-create-tables.sql /docker-entrypoint-initdb.d/
COPY ./03-create-procedures.sql /docker-entrypoint-initdb.d/
COPY ./04-create-admin-users.sql /docker-entrypoint-initdb.d/
COPY ./05-create-public-users.sql /docker-entrypoint-initdb.d/
