#!/bin/bash

# wait until chain is running
until curl http://localhost:26657/abci_info? 2>/dev/null | grep -q 'last_block_height'
do
  echo "Waiting for local migaloo to start"
  sleep 2
done

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# upload furnace contract
res=$(MOUNT_VOLUME="-v $SCRIPT_DIR/../artifacts:/artifacts" "$SCRIPT_DIR"/migaloo.sh migalood tx wasm store artifacts/furnace.wasm --from test1 --keyring-backend test --chain-id=local-1 --gas=auto --gas-adjustment 1.3  --broadcast-mode=block -y  )

code_id=$(echo "$res" | yq '.logs[0].events[] | select(.type == "store_code").attributes[] | select(.key == "code_id").value')

echo "code_id =" $code_id

# instantiate furnace contract
res=$("$SCRIPT_DIR"/migaloo.sh migalood tx wasm instantiate "$code_id" '{}' --label "furnace" --no-admin --from test1 --keyring-backend test --chain-id=local-1 --gas=auto --gas-adjustment 1.3  --broadcast-mode=block --amount 100uwhale -y)


address=$(echo "$res" | yq '.logs[0].events[] | select(.type == "instantiate").attributes[] | select(.key == "_contract_address").value')

echo "contract address" "$address"