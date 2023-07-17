import React from "react";
import { Button, HStack, Text, useToast } from "@chakra-ui/react";
import { WalletClientType } from "@/types/wallet-client";
import { useKeplrInterface } from "@/hooks/useKeplrInterface";
import WalletIcon from "../shared/WalletIcon";
import { useTerraStation } from "@/hooks/useTerraStation";

type ConnectButtonProps = {
  type: WalletClientType;
};

const ConnectButton = ({ type }: ConnectButtonProps) => {
  const toast = useToast();

  const onError = (e: unknown) => {
    console.log(`Error connecting to ${type}: ${e}`);

    const id = "wallet-connection-error";

    if (!toast.isActive(id)) {
      toast({
        description: `Failed to connect to ${type}.`,
        id: id,
        duration: 10000,
        position: "top",
        status: "error",
        isClosable: true,
      });
    }
  };

  const connectToCosmostation = useKeplrInterface(
    WalletClientType.cosmostation,
    window.cosmostation,
    onError
  );

  const connectToKeplr = useKeplrInterface(
    WalletClientType.keplr,
    window.keplr,
    onError
  );

  const connectToLeap = useKeplrInterface(
    WalletClientType.leap,
    window.leap,
    onError
  );

  const connectToTerraStation = useTerraStation(onError);

  const getConnect = () => {
    switch (type) {
      case WalletClientType.cosmostation:
        return connectToCosmostation;
      case WalletClientType.keplr:
        return connectToKeplr;
      case WalletClientType.leap:
        return connectToLeap;
      case WalletClientType.terrastation:
        return connectToTerraStation;
    }
  };

  return (
    <Button onClick={getConnect()} variant="modal">
      <HStack flex={1} justifyContent="left" gap={4}>
        <WalletIcon type={type} />
        <Text align="left" fontWeight={600}>
          Connect {type}
        </Text>
      </HStack>
    </Button>
  );
};

export default ConnectButton;
