import {CosmWasmClient} from "@cosmjs/cosmwasm-stargate";
import {useQuery} from "react-query";
import chain from "../../public/config.json";

export function useCosmWasmClient(): CosmWasmClient | undefined {

    const {data: client} = useQuery(
        ['cosmwasmClient'],
        async () => {
            return await CosmWasmClient.connect(chain.chain_info.rpc)
        },
    )
    return client
}
