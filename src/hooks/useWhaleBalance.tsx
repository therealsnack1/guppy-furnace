import {
  burningState,
  walletClientState,
  whaleBalanceState,
  realwhaleBalanceState,
} from "@/components/shared/state";
import { useEffect } from "react";
import { useRecoilState, useRecoilValue } from "recoil";

export const useRealWhaleBalance = () => {
  const [whaleBalance, setWhaleBalance] = useRecoilState(realwhaleBalanceState);

  const walletClient = useRecoilValue(walletClientState);
  const burningStateValue = useRecoilValue(burningState);

  useEffect(() => {
    const getBalance = async () => {
      if (walletClient != null) {
        const newBalance = await walletClient!.getWhaleRealBalance();
        setWhaleBalance(newBalance);
      } else {
        setWhaleBalance(0);
      }
    };

    getBalance();
  }, [burningStateValue, walletClient]);

  return whaleBalance;
};

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