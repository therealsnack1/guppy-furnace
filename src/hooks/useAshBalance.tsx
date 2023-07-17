import {
  ashBalanceState,
  burningState,
  walletClientState,
} from "@/components/shared/state";
import { useEffect } from "react";
import { useRecoilState, useRecoilValue } from "recoil";

export const useAshBalance = () => {
  const [ashBalance, setAshBalance] = useRecoilState(ashBalanceState);

  const walletClient = useRecoilValue(walletClientState);
  const burningStateValue = useRecoilValue(burningState);

  useEffect(() => {
    const getBalance = async () => {
      if (walletClient != null) {
        const newBalance = await walletClient!.getAshBalance();
        setAshBalance(newBalance);
      } else {
        setAshBalance(0);
      }
    };

    getBalance();
  }, [burningStateValue, walletClient]);

  return ashBalance;
};
