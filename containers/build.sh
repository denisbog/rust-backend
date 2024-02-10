docker build -t build .
docker run --rm -v${PWD}:/app build
