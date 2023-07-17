# White Whale Furnace

## Quickstart

This will launch a local migaloo instance, compile and upload the burn contract, and starts the frontend.

```shell
npm install
docker compose up
```

Run optional service to create several temporary accounts that burn Whale

```shell
# run this instead of above command
docker-compose --profile burner up
# Or if local chain is running already
docker-compose up burner-accounts
```

## Development

### Build contract

```shell
./scripts/optimize.sh
```

### Running contract on local migaloo chain

Building the docker file

```shell
./scripts/build_migaloo_container.sh
```

To start the chain in a docker container

```bash
./scripts/start_local.sh
```

Upload furnace contract to local chain. If successful the script will return the contract address.

```shell
./scripts/upload_to_local.sh
```

### Configuring wallet to use local migaloo

To configure the instance of migaloo you are connecting to, modify the chain info to the config.json in the public folder. The chain info should use [Keplr's interface](https://docs.keplr.app/api/suggest-chain.html). You can validate your chain info json format [here](https://docs.axelar.dev/resources/keplr). The default is to connect to the local migaloo instance started in the docker container.

The furnace contract address will be generated and updated in the config file at build time if using the docker quickstart command, else it must be manually edited with after uploading the furnance contract.

The API endpoint for querying the leaderboard data should support paginated queries and return a response with data in the below JSON format, with nodes in descending order by totalBurn. The default is the endpoint set up after running the quickstart command.

```yaml
{
  "data": {
      "sumBurns":{
         "nodes":[
            {
               "id":"migaloo19vs2t96yfjf22aa8ps9r4cemzkvz07c067zsvv",
               "totalBurn":"28000000"
            },
            {
               "id":"migaloo18rtnjxy6zt2xf24ln0m77re937vgl7kd8x7474",
               "totalBurn":"4000000"
            }
            ...
         ]
      }
}
```

Example paginated query:

```bash
http://localhost:3050/?query={sumBurns(orderBy:TOTAL_BURN_DESC,first:10,offset:0){nodes{id%20totalBurn}}}
```

### Frontend Development

```bash
npm install
npm run dev
```

### Frontend Production

```bash
npm install
npm run build
npm run start
```
