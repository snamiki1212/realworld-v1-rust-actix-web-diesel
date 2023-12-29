#!/usr/bin/env bash
set -x

# SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

APIURL=${APIURL:-https://api.realworld.io/api}
USERNAME=${USERNAME:-u`date +%s`}
EMAIL=${EMAIL:-$USERNAME@mail.com}
PASSWORD=${PASSWORD:-password}

# Show env
echo "showing env..."
echo "APIURL=$APIURL"
echo "USERNAME=$USERNAME"
echo "EMAIL=$EMAIL"
echo "PASSWORD=$PASSWORD" 

# Ready for E2E
npm install -g newman

# healthcheck
sh ./x.sh

# Run E2E test
echo "running e2e..."
npx newman run ./e2e/Conduit.postman_collection.json \
  --delay-request 500 \
  --global-var "APIURL=$APIURL" \
  --global-var "USERNAME=$USERNAME" \
  --global-var "EMAIL=$EMAIL" \
  --global-var "PASSWORD=$PASSWORD" \
  "$@"