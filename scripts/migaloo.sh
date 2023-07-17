#!/bin/bash
# open ports, and add name so other containers can interact.
if [ "$RUN_LOCAL" == "true" ]; then
  RUN_LOCAL_ARGS="-p 1317:1317 -p 26657:26657 --name migaloo_local"
else
  RUN_LOCAL_ARGS=""
fi

if [ "$INTERACTIVE" == "true" ]; then
  INTERACTIVE_ARG="-it"
else
  INTERACTIVE_ARG=""
fi

# make docker network
docker network create --driver bridge 2>/dev/null

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

docker run --rm $INTERACTIVE_ARG --network local-migaloo $MOUNT_VOLUME -v "$SCRIPT_DIR"/../.migalood:/root/.migalood $RUN_LOCAL_ARGS migaloo-local $@


