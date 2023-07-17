#! /bin/sh

export HOME_DIR=$(eval echo "${HOME_DIR:-"~/.migalood"}")
export BINARY=${BINARY:-migalood}
export MONIKER="localmagaloo"
export CHAIN_ID=${CHAIN_ID:-"local-1"}
export KEY="migaloo1"
export KEYALGO="secp256k1"
export KEYRING=${KEYRING:-"test"}

export RPC=${RPC:-"26657"}
export REST=${REST:-"1317"}
export PROFF=${PROFF:-"6060"}
export P2P=${P2P:-"26656"}
export GRPC=${GRPC:-"9090"}
export GRPC_WEB=${GRPC_WEB:-"9091"}
export ROSETTA=${ROSETTA:-"8080"}
export TIMEOUT_COMMIT=${TIMEOUT_COMMIT:-"5s"}


# address: migaloo1psn37yxhpuv3jjrtwl9zxajxf7sxfz39xe7s6y
#  name: magiloo1
echo  "universe allow swim bonus icon mass flavor remind swing flee lazy stamp three employ exit mixture diamond flash claw umbrella couch entire drill gentle" | $BINARY keys add $KEY --keyring-backend $KEYRING --algo $KEYALGO --recover

# address: migaloo1nadgu2usmt20v48nu3cwkv9jljkvewra7kms8w
# name: feeacc
echo "eye unusual rib prevent rule fortune process meadow gloom grace race any walk fruit erupt thought error velvet trash good caught idle harbor armor" | $BINARY keys add feeacc --keyring-backend $KEYRING --algo $KEYALGO --recover

# address: migaloo18rtnjxy6zt2xf24ln0m77re937vgl7kd8x7474
# name: test1
echo "prepare poem blame radio sting method mass scare romance captain misery task fresh soap motion cat club grocery resemble breeze hard replace hungry gloom" | $BINARY keys add test1 --keyring-backend $KEYRING --algo $KEYALGO --recover

# address: migaloo19vs2t96yfjf22aa8ps9r4cemzkvz07c067zsvv
# name: test2
echo "hedgehog depth party buzz sponsor vault skill bamboo oak amateur list usage dirt bag dilemma maple sun exact predict genuine retreat giraffe country merge" | $BINARY keys add test2 --keyring-backend $KEYRING --algo $KEYALGO --recover

# address: migaloo1rpgc24c0xa2n2y3emy45e4max6t8zjymhelghu
# name: test3
echo "gloom explain again hurdle duty lazy unaware become south slow share actress saddle viable skill measure hotel combine actor blast invest tip slogan clerk" | $BINARY keys add test3 --keyring-backend $KEYRING --algo $KEYALGO --recover

# address: migaloo1qml3n560hayvmkmrtxmq2a49a4hu8c6lr5due2
# name: test4
echo "crowd margin better impulse myself stuff perfect gas grow panther debate scan disagree level tooth park eternal include strike grunt hazard seat inmate frown" | $BINARY keys add test4 --keyring-backend $KEYRING --algo $KEYALGO --recover

$BINARY init $MONIKER --chain-id $CHAIN_ID


# Function updates the config based on a jq argument as a string
update_test_genesis () {
  # update_test_genesis '.consensus_params["block"]["max_gas"]="100000000"'
  cat $HOME_DIR/config/genesis.json | jq "$1" > $HOME_DIR/config/tmp_genesis.json && mv $HOME_DIR/config/tmp_genesis.json $HOME_DIR/config/genesis.json
}

# Set gas limit in genesis
update_test_genesis '.consensus_params["block"]["max_gas"]="100000000"'
update_test_genesis '.app_state["gov"]["voting_params"]["voting_period"]="15s"'

# GlobalFee
update_test_genesis '.app_state["globalfee"]["params"]["minimum_gas_prices"]=[{"amount":"0.002500000000000000","denom":"uwhale"}]'

update_test_genesis '.app_state["staking"]["params"]["bond_denom"]="uwhale"'
update_test_genesis '.app_state["bank"]["params"]["send_enabled"]=[{"denom": "uwhale","enabled": true}]'
# update_test_genesis '.app_state["staking"]["params"]["min_commission_rate"]="0.100000000000000000"' # sdk 46 only

update_test_genesis '.app_state["mint"]["params"]["mint_denom"]="uwhale"'
update_test_genesis '.app_state["gov"]["deposit_params"]["min_deposit"]=[{"denom": "uwhale","amount": "1000000"}]'
update_test_genesis '.app_state["crisis"]["constant_fee"]={"denom": "uwhale","amount": "1000"}'

update_test_genesis '.app_state["tokenfactory"]["params"]["denom_creation_fee"]=[{"denom":"uwhale","amount":"100"}]'

update_test_genesis '.app_state["feeshare"]["params"]["allowed_denoms"]=["uwhale"]'

# Allocate genesis accounts
$BINARY add-genesis-account $KEY 1000000000uwhale,1000utest --keyring-backend $KEYRING
$BINARY add-genesis-account feeacc 100000000uwhale,1000utest --keyring-backend $KEYRING

$BINARY add-genesis-account test1 100000000uwhale,1000utest --keyring-backend $KEYRING
$BINARY add-genesis-account test2 100000000uwhale,1000utest --keyring-backend $KEYRING

$BINARY gentx $KEY 1000000uwhale --keyring-backend $KEYRING --chain-id $CHAIN_ID

# Collect genesis tx
$BINARY collect-gentxs

# Run this to ensure that the genesis file is setup correctly
$BINARY validate-genesis

# Opens the RPC endpoint to outside connections
sed -i 's/laddr = "tcp:\/\/127.0.0.1:26657"/c\laddr = "tcp:\/\/0.0.0.0:'$RPC'"/g' $HOME_DIR/config/config.toml
sed -i 's/cors_allowed_origins = \[\]/cors_allowed_origins = \["\*"\]/g' $HOME_DIR/config/config.toml

# REST endpoint
sed -i 's/address = "tcp:\/\/0.0.0.0:1317"/address = "tcp:\/\/0.0.0.0:'$REST'"/g' $HOME_DIR/config/app.toml
sed -i 's/enable = false/enable = true/g' $HOME_DIR/config/app.toml

sed -i 's/swagger = false/swagger = true/g' $HOME_DIR/config/app.toml

# replace pprof_laddr = "localhost:6060" binding
sed -i 's/pprof_laddr = "localhost:6060"/pprof_laddr = "localhost:'$PROFF_LADDER'"/g' $HOME_DIR/config/config.toml

# change p2p addr laddr = "tcp://0.0.0.0:26656"
sed -i 's/laddr = "tcp:\/\/0.0.0.0:26656"/laddr = "tcp:\/\/0.0.0.0:'$P2P'"/g' $HOME_DIR/config/config.toml

# GRPC
sed -i 's/address = "0.0.0.0:9090"/address = "0.0.0.0:'$GRPC'"/g' $HOME_DIR/config/app.toml
sed -i 's/address = "0.0.0.0:9091"/address = "0.0.0.0:'$GRPC_WEB'"/g' $HOME_DIR/config/app.toml

# Rosetta Api
sed -i 's/address = ":8080"/address = "0.0.0.0:'$ROSETTA'"/g' $HOME_DIR/config/app.toml

# faster blocks
sed -i 's/timeout_commit = "5s"/timeout_commit = "'$TIMEOUT_COMMIT'"/g' $HOME_DIR/config/config.toml

# remove seed nodes
sed -i 's/seeds = ".*"/seeds = ""/g' $HOME_DIR/config/config.toml

# set client to connect to local chain
sed -i 's/node = ".*"/node = "tcp:\/\/migaloo_local:26657"/g' $HOME_DIR/config/client.toml
