import { CosmWasmWalletClient, WalletClientType } from "@/types/wallet-client";
import { Keplr } from "@keplr-wallet/types";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { useRecoilState } from "recoil";
import { walletClientState } from "@/components/shared/state";
import { useConfig } from "./useConfig";
import { Tendermint34Client } from "@cosmjs/tendermint-rpc";
import { QueryClient, setupBankExtension } from "@cosmjs/stargate";

export const useKeplrInterface = (
  type: WalletClientType,
  keplr: Keplr,
  onError: (e: unknown) => void
) => {
  const [_, setWalletClient] = useRecoilState(walletClientState);
  const { chainInfo, furnaceContractAddress } = useConfig();

  return async () => {
    if (!keplr) {
      alert(`Please install ${type} extension and refresh the page.`);
    } else {
      try {
        await keplr.experimentalSuggestChain(chainInfo);

        await keplr.enable(chainInfo.chainId);

        const offlineSigner = keplr.getOfflineSigner(chainInfo.chainId);

        const accounts = await offlineSigner.getAccounts();

        const client = await SigningCosmWasmClient.connectWithSigner(
          chainInfo.rpc,
          offlineSigner
        );

        const address = accounts[0].address;

        if (address == undefined || !address.startsWith("migaloo")) {
          throw new Error("Failed to get migaloo address from wallet");
        }

        const tendermintClient = await Tendermint34Client.connect(
          chainInfo.rpc
        );

        const queryClient = new QueryClient(tendermintClient);

        const bankExtension = setupBankExtension(queryClient);

        const cosmWasmWalletClient = new CosmWasmWalletClient(
          client,
          bankExtension,
          type,
          address,
          chainInfo,
          furnaceContractAddress
        );

        setWalletClient(cosmWasmWalletClient);
      } catch (e) {
        onError(e);
      }
    }
  };
};
