FROM mysql:latest

# Set environment variables for MySQL setup
ENV MYSQL_ROOT_PASSWORD=REG+~.Uqj1HYuci_0aFUht~Nk56Vh
ENV MYSQL_DATABASE=masjidappdatabase

ENV MYSQL_AUTHENTICATION_USER=authenticationuser
ENV MYSQL_AUTHENTICATION_USER_PASSWORD=BL/6Fx$Ku!2{37GvPS9



# Copy custom initialization scripts into the container
COPY ./scripts /docker-entrypoint-initdb.d/

# Expose the default MySQL port (optional, for documentation purposes)
EXPOSE 3306
