#!/bin/sh

# wait for keychain
until migalood keys show test1 --keyring-backend test
do
  echo "Waiting for local migaloo to keychain"
  sleep 2
done

sleep 3

# upload furnace contract
res=$(migalood tx wasm store artifacts/furnace.wasm --from test1 --keyring-backend test --chain-id=local-1 --gas=auto --gas-adjustment 1.3  --broadcast-mode=block -y --node tcp://local-migaloo:26657 )
echo "$res"

code_id=$(echo "$res" | yq '.logs[0].events[] | select(.type == "store_code").attributes[] | select(.key == "code_id").value')

echo "code_id =" $code_id

# instantiate furnace contract
res=$(migalood tx wasm instantiate "$code_id" '{}' --label "furnace" --no-admin --from test1 --keyring-backend test --chain-id=local-1 --gas=auto --gas-adjustment 1.3  --broadcast-mode=block --amount 100uwhale -y --node tcp://local-migaloo:26657 )


address=$(echo "$res" | yq '.logs[0].events[] | select(.type == "instantiate").attributes[] | select(.key == "_contract_address").value')

echo "contract address" "$address"
echo "$(jq --arg address "$address" '.furnace_contract_address=$address' ./public/config.json)" > ./public/config.json
# echo "CONTRACT_ADDRESS=$address" > /.env.development.local
