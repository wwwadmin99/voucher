#!/bin/sh
set -e

URL="http://localhost:8080"

echo "Запуск проекта через $URL ..."

docker compose up --build -d

echo "Waiting for $URL ..."
until curl -sf "$URL" > /dev/null 2>&1; do
  sleep 1
done

if command -v xdg-open > /dev/null 2>&1; then
  xdg-open "$URL" > /dev/null 2>&1 &
elif command -v open > /dev/null 2>&1; then
  open "$URL"
fi

echo "$URL is up. Following logs (Ctrl+C to stop watching; containers keep running — use \`npm stop\` to tear down)."
docker compose logs -f
