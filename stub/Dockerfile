FROM wiremock/wiremock:latest
LABEL authors="sfe34"

COPY --chown=65534 /mappings/*.json /home/wiremock/mappings

EXPOSE 8080

ENTRYPOINT ["/docker-entrypoint.sh", "--global-response-templating", "--disable-gzip", "--verbose"]
