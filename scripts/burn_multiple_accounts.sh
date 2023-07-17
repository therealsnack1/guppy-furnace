#! /bin/sh

# wait for keychain
until migalood keys show test1 --keyring-backend test
do
  echo "Waiting for local migaloo to keychain"
  sleep 2
done

# get contract address from config.json
until [ ! -z $FURNACE ]
do
  # get contract address from config.json
  FURNACE=$(cat /public/config.json | jq -r '."furnace_contract_address"')
  echo "waiting for furnace contract upload"
  sleep 10
done

for n in $(seq 1 10)
do
  addr=$(migalood keys add burner"$n" --keyring-backend test --output json | jq -r '."address"')
  echo "address" $addr
  migalood tx bank send test2 "$addr" "$n"0000uwhale --keyring-backend test --chain-id=local-1 -y --node tcp://local-migaloo:26657
  #sleep for at least one block for hte transfer to take effect
  sleep 10
  migalood tx wasm execute $FURNACE '{"burn":{}}' --from burner"$n" --keyring-backend test --chain-id=local-1 --gas=auto --gas-adjustment 1.3  --broadcast-mode=block --amount "$n"0000uwhale -y --output json --node tcp://local-migaloo:26657
  migalood keys delete burner"$n" --keyring-backend test -y
done
