#!/bin/bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

#run config if local chain is not setup
if [[ ! -d $SCRIPT_DIR/../.migalood ]]; then
  "$SCRIPT_DIR"/migaloo.sh sh /config.sh
fi

# start local chain
export RUN_LOCAL=true
"$SCRIPT_DIR"/migaloo.sh