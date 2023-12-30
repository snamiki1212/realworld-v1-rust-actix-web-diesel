#!/usr/bin/env bash

HOST="10.0.0.0:8080"
RETRY=10
SLEEP=1
CURL_TIMEOUT=3 #sec
# RESULT=`curl --max-time $CURL_TIMEOUT http://${HOST}/api/healthcheck`

for i in {1..10}; do
  echo "-------------------"
  
  # run curl
  echo "running curl..."
  RESULT=`curl --max-time $CURL_TIMEOUT http://${HOST}/api/healthcheck`

  # check
  if [ "$RESULT" == "OK" ]; then
    echo "API is ready"
    break
  fi
  echo "API is not ready, retrying..."

  # docker compose logs

  # sleep
  sleep $SLEEP
done
