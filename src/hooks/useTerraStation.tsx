import { walletClientState } from "@/components/shared/state";
import { TerraStationWalletClient } from "@/types/wallet-client";
import {
  ConnectType,
  useConnectedWallet,
  useWallet,
} from "@terra-money/wallet-provider";
import { useRecoilState } from "recoil";
import { LCDClient } from "@terra-money/feather.js";
import { useConfig } from "./useConfig";

export const useTerraStation = (onError: (e: unknown) => void) => {
  const [walletClient, setWalletClient] = useRecoilState(walletClientState);
  const { chainInfo, furnaceContractAddress } = useConfig();

  const { connect, disconnect } = useWallet();
  const connectedWallet = useConnectedWallet();

  return async () => {
    try {
      const chainId = chainInfo.chainId;

      const lcdClient = new LCDClient({
        [chainId]: {
          lcd: chainInfo.rest,
          chainID: chainId,
          gasAdjustment: 0.1,
          gasPrices: {
            uwhale: chainInfo.feeCurrencies[0].gasPriceStep?.low ?? 0,
          },
          prefix: "migaloo",
        },
      });

      connect(ConnectType.EXTENSION, "station");

      const address = connectedWallet?.addresses[chainId];

      if (connectedWallet == undefined) {
        throw new Error("Failed to get connected wallet");
      }

      if (address == undefined || !address.startsWith("migaloo")) {
        throw new Error("Failed to get migaloo address from wallet");
      }

      const terraStationWalletClient = new TerraStationWalletClient(
        lcdClient,
        connectedWallet!,
        disconnect,
        address!,
        chainInfo,
        furnaceContractAddress
      );

      setWalletClient(terraStationWalletClient);
    } catch (e) {
      onError(e);
    }
  };
};
