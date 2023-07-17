import {
  Button,
  HStack,
  Icon,
  StackDivider,
  useDisclosure,
  Text,
  Box,
  useClipboard,
  useToast,
} from "@chakra-ui/react";
import { IoMdWallet, IoMdCloseCircle } from "react-icons/io";
import ConnectModal from "../connect/ConnectModal";
import { useRecoilState } from "recoil";
import { walletClientState } from "../shared/state";
import { useWhaleBalance } from "@/hooks/useWhaleBalance";
import { truncateAddress } from "@/utils/truncateAddress";
import WalletIcon from "../shared/WalletIcon";
import { useConfig } from "@/hooks/useConfig";
import { ChainInfo } from "@keplr-wallet/types";

const WalletInfo = () => {
  const { isOpen, onOpen, onClose } = useDisclosure();
  const [walletClient, setWalletClient] = useRecoilState(walletClientState);
  const balance = useWhaleBalance();

  const disconnectAndResetWalletState = async () => {
    await walletClient?.disconnect();
    setWalletClient(null);
    onClose();
  };

  const { onCopy, hasCopied } = useClipboard(walletClient?.address ?? "");

  const toast = useToast();

  const onCopyAddress = () => {
    onCopy();

    const id = "copy-address";

    if (!toast.isActive(id) && walletClient != null) {
      toast({
        description: "Address copied to clipboard",
        id: id,
        duration: 2000,
        position: "top-right",
        status: "success",
        isClosable: true,
        variant: "toast",
      });
    }
  };

  const { chainInfo, furnaceContractAddress } = useConfig();

  function isInstanceOfChainInfo(object: any): object is ChainInfo {
    return (
      "chainId" in object &&
      "chainName" in object &&
      "rest" in object &&
      "rpc" in object &&
      "bip44" in object &&
      "bech32Config" in object &&
      "currencies" in object &&
      "stakeCurrency" in object &&
      "feeCurrencies" in object
    );
  }

  const onConnectWallet = () => {
    try {
      if (chainInfo == undefined) {
        throw new Error("chain_info is missing");
      }

      if (!isInstanceOfChainInfo(chainInfo)) {
        throw new Error("chain_info is incorrectly formatted");
      }

      if (
        furnaceContractAddress == undefined ||
        furnaceContractAddress.length == 0
      ) {
        throw new Error("furnace_contract_address is missing");
      }

      if (
        typeof furnaceContractAddress !== "string" ||
        !furnaceContractAddress.startsWith("migaloo1")
      ) {
        throw new Error("furnace_contract_address is incorrectly formatted");
      }

      onOpen();
    } catch (e) {
      console.log(`Bad config.json format: ${e}`);
      const id = "config-error";

      if (!toast.isActive(id)) {
        toast({
          description: "Your config.json file is incorrectly formatted.",
          id: id,
          duration: 2000,
          position: "top",
          status: "error",
          isClosable: true,
        });
      }
    }
  };

  return walletClient != null ? (
    <HStack divider={<StackDivider borderColor="lightTransparentGray" />}>
      <Text
        fontSize={15}
        color="white"
        fontFamily={`'Lato', sans-serif`}
        padding={2}
      >
        {`${balance.toFixed(3)} WHALE`}
      </Text>
      <HStack>
        <Box paddingLeft={2}>
          <WalletIcon type={walletClient.type} />
        </Box>
        <Text
          fontSize={15}
          color="white"
          fontFamily={`'Lato', sans-serif`}
          paddingTop={2}
          paddingBottom={2}
          paddingLeft={2}
          onClick={onCopyAddress}
          as="button"
        >
          {walletClient.address !== undefined &&
            truncateAddress(walletClient.address)}
        </Text>
        <Icon
          as={IoMdCloseCircle}
          color="brandGreen"
          boxSize={5}
          onClick={disconnectAndResetWalletState}
        />
      </HStack>
    </HStack>
  ) : (
    <HStack>
      <Button variant="navbar" gap="2" onClick={onConnectWallet}>
        <Icon as={IoMdWallet} boxSize={6} /> Connect Wallet
      </Button>
      <ConnectModal isOpen={isOpen} onClose={onClose} />
    </HStack>
  );
};

export default WalletInfo;
