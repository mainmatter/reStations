#!/usr/bin/env bash

# This is just for Heroku, it's NOT the entrypoint for the Dockerfile.
# You can see it used in heroku.yml

# Heroku requires us to re-bind some variables provided only at boot time for the server to run.
export APP_SERVER__PORT="$PORT"
export APP_SERVER__IP="0.0.0.0"

/usr/local/bin/restations-web
