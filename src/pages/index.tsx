import React, { useState } from "react";
import { useToast } from "@chakra-ui/react";
import BurnForm from "@/components/burner/BurnForm";
import {
  BurningState,
  burningState,
  walletClientState,
} from "@/components/shared/state";
import { useRecoilState, useRecoilValue } from "recoil";
import { useWhaleBalance } from "@/hooks/useWhaleBalance";
import BurnPage from "@/components/burner/BurnPage";
import SuccessDetails from "@/components/burner/SuccessDetails";
import { useAshBalance } from "@/hooks/useAshBalance";

export default function Home() {
  const walletClient = useRecoilValue(walletClientState);

  const isConnected = walletClient != null;

  const [burningStateValue, setBurningStateValue] =
    useRecoilState(burningState);

  const whaleBalance = useWhaleBalance();
  const ashBalance = useAshBalance();

  const [hasInsufficientWhale, setHasInsufficientWhale] = useState(false);
  const [whalesToBurn, setWhalesToBurn] = useState(0);
  const [whalesBurned, setWhalesBurned] = useState(0);

  const toast = useToast();

  const handleChange = (valueAsNumber: number) => {
    setWhalesToBurn(valueAsNumber);

    if (walletClient != null) {
      if (whaleBalance !== undefined && (valueAsNumber ?? 0) > whaleBalance) {
        setHasInsufficientWhale(true);
      } else {
        setHasInsufficientWhale(false);
      }
    }
  };

  const onBurnWhales = async () => {
    try {
      if (!hasInsufficientWhale && whalesToBurn > 0 && walletClient != null) {
        const currentAshBalance = ashBalance;

        setBurningStateValue(BurningState.burning);

        setWhalesBurned(whalesToBurn);

        await walletClient!.burn(whalesToBurn);

        setWhalesToBurn(0);

        let interval = setInterval(async () => {
          const newAshBalance = await walletClient.getAshBalance();
          if (newAshBalance > currentAshBalance) {
            clearInterval(interval);
          }
        }, 5000);

        setBurningStateValue(BurningState.success);
      }
    } catch (e) {
      console.log(`Error burning WHALE: ${e}`);
      setBurningStateValue(BurningState.userInput);

      const id = "burning-error";

      if (!toast.isActive(id)) {
        toast({
          description:
            "Error burning WHALE. Make sure you confirm the transaction from your wallet extension.",
          id: id,
          duration: 10000,
          position: "top",
          status: "error",
          isClosable: true,
        });
      }
    }
  };

  const onClickBack = async () => {
    setBurningStateValue(BurningState.userInput);
  };

  switch (burningStateValue) {
    case BurningState.userInput:
      return (
        <BurnPage
          buttonText="Burn WHALE"
          onClick={onBurnWhales}
          isDisabled={!isConnected}
        >
          <BurnForm
            handleChange={handleChange}
            hasInsufficientWhale={hasInsufficientWhale}
          />
        </BurnPage>
      );
    case BurningState.burning:
      return <BurnPage buttonText="Burning WHALE..." />;
    case BurningState.success:
      return (
        <BurnPage buttonText="Back to Burn" onClick={onClickBack}>
          <SuccessDetails whaleBurned={whalesBurned} />
        </BurnPage>
      );
  }
}
