import React from "react";
import { HStack, Box, Button } from "@chakra-ui/react";
import WalletInfo from "./WalletInfo";
import { useRouter } from "next/router";
import { BurningState, burningState } from "@/components/shared/state";
import { useRecoilState } from "recoil";

const Navbar = () => {
  const { push, pathname } = useRouter();
  const [_, setBurningStateValue] =
    useRecoilState(burningState);

  const onClickBurner = () => {
    setBurningStateValue(BurningState.userInput);

    push("/");
  };
  const onClickLeaderboard = () => {
    setBurningStateValue(BurningState.userInput);
    push("/leaderboard");
  };

  return (
    <HStack
      minWidth="100%"
      justifyContent="right"
      paddingTop={30}
      paddingBottom={41}
      paddingRight={12}
      gap={5}
    >
      <Button
        variant="secondary"
        onClick={onClickBurner}
        borderWidth="2px"
        borderColor={
          pathname.includes("leaderboard") ? "transparent" : "brandGreen"
        }
      >
        Burner
      </Button>
      <Button
        variant="secondary"
        onClick={onClickLeaderboard}
        borderWidth="2px"
        borderColor={
          pathname.includes("leaderboard") ? "brandGreen" : "transparent"
        }
      >
        Leaderboard
      </Button>
      <Box
        bg="darkTransparentGray"
        paddingLeft={4}
        paddingRight={4}
        borderRadius="100px"
      >
        <WalletInfo />
      </Box>
    </HStack>
  );
};

export default Navbar;
