#!/usr/bin/env bash

source .envrc

# set -x

AUTH_BODY=$(jq --null-input \
  --arg client_id "${AUTH0_CLIENT_ID}" \
  --arg client_secret "${AUTH0_CLIENT_SECRET}" \
  --arg audience "${AUTH0_AUDIENCE}" \
  --arg grant_type "${AUTH0_GRANT_TYPE}" \
  '{"client_id": $client_id, "client_secret": $client_secret, "audience": $audience, "grant_type": $grant_type}')

AUTH_RESP=$(curl -s --request POST \
  --url https://${AUTH0_DOMAIN}/oauth/token \
  --header 'content-type: application/json' \
  --data "${AUTH_BODY}")

AUTH_TOKEN=$(echo "${AUTH_RESP}" | jq -r .access_token)

curl -i --request GET \
  --url ${SPIN_APP_URL} \
  --header "authorization: Bearer ${AUTH_TOKEN}"
