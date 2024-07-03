FROM gclub/skywardai:v0.1.14

WORKDIR /app

COPY . .

EXPOSE 8083
ENTRYPOINT [ "entrypoint.sh" ]