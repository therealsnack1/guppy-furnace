import { WalletClientType } from "@/types/wallet-client";
import { Image } from "@chakra-ui/react";

type WalletIconProps = {
  type: WalletClientType;
};

const WalletIcon = ({ type }: WalletIconProps) => {
  const iconUrl = `icons/${type.toLowerCase().replace(" ", "")}-icon.svg`;

  return <Image src={iconUrl} width="17px" alt="" />;
};

export default WalletIcon;
