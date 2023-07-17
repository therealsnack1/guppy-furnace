import {
  burningState,
  walletClientState,
  whaleBalanceState,
} from "@/components/shared/state";
import { useEffect } from "react";
import { useRecoilState, useRecoilValue } from "recoil";

export const useWhaleBalance = () => {
  const [whaleBalance, setWhaleBalance] = useRecoilState(whaleBalanceState);

  const walletClient = useRecoilValue(walletClientState);
  const burningStateValue = useRecoilValue(burningState);

  useEffect(() => {
    const getBalance = async () => {
      if (walletClient != null) {
        const newBalance = await walletClient!.getWhaleBalance();
        setWhaleBalance(newBalance);
      } else {
        setWhaleBalance(0);
      }
    };

    getBalance();
  }, [burningStateValue, walletClient]);

  return whaleBalance;
};
