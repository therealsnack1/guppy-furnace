import {
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalBody,
  ModalCloseButton,
  VStack,
  Text,
  ModalFooter,
} from "@chakra-ui/react";
import { WalletClientType } from "@/types/wallet-client";
import ConnectButton from "./ConnectButton";

interface ConnectModalProps {
  isOpen: boolean;
  onClose: () => void;
}

const ConnectModal = ({ isOpen, onClose }: ConnectModalProps) => {
  const availableWallets: WalletClientType[] = [];

  if (typeof window !== "undefined") {
    if (window.cosmostation) {
      availableWallets.push(WalletClientType.cosmostation);
    }

    if (window.keplr) {
      availableWallets.push(WalletClientType.keplr);
    }

    if (window.leap) {
      availableWallets.push(WalletClientType.leap);
    }

    if (window.isTerraExtensionAvailable) {
      availableWallets.push(WalletClientType.terrastation);
    }
  }

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>Select Wallet</ModalHeader>
        <ModalCloseButton />
        <ModalBody>
          {availableWallets.length > 0 ? (
            <VStack gap={2}>
              {availableWallets.map((walletType) => {
                return <ConnectButton type={walletType} key={Math.random()} />;
              })}
            </VStack>
          ) : (
            <Text textAlign="center">
              No wallet extensions available. Install Cosmostation, Keplr, Leap,
              or Terra Station to connect your wallet.
            </Text>
          )}
        </ModalBody>
        <ModalFooter />
      </ModalContent>
    </Modal>
  );
};

export default ConnectModal;
