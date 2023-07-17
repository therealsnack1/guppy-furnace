import {
  VStack,
  Center,
  Heading,
  Button,
  Text,
  Box,
  Flex,
  Spacer,
} from "@chakra-ui/react";
import EllipticalShadow from "./EllipticalShadow";
import Background from "../shared/Background";
import Navbar from "../navbar/Navbar";
import { MouseEventHandler, ReactNode } from "react";
import { useTotalWhaleBurned } from "@/hooks/useTotalWhaleBurned";

type DefaultBurnProps = {
  buttonText: string;
  onClick?: MouseEventHandler<HTMLButtonElement> | undefined;
  isDisabled?: boolean;
  children?: ReactNode;
};

const BurnPage = ({
  buttonText,
  onClick,
  isDisabled,
  children,
}: DefaultBurnProps) => {

  const totalWhaleBurned = useTotalWhaleBurned();

  return (
    <Background>
      <Flex direction="column" height="100vh">
        <Navbar />
        <Spacer />
        <Center>
          <EllipticalShadow>
            <VStack flex={4}>
              <Text color="brandGreen" fontWeight={500} fontSize={20}>
                WHALE Burner
              </Text>
              <Heading fontWeight={900} color="white" fontSize={72}>
                The Furnace
              </Heading>
              <Box width={500} minHeight="255px">
                {children}
              </Box>
              <Button
                variant="primary"
                onClick={onClick}
                isDisabled={isDisabled}
              >
                {buttonText}
              </Button>
            </VStack>
          </EllipticalShadow>
        </Center>
        <Spacer />
        <Center minHeight="100px">
            <Flex>
              <Box
                width="550px"
                background="rgba(0, 0, 0, 0.8)"
                padding={5}
                borderRadius="10px"
              >
                <Flex>
                  <Heading flex={13} color="white" fontSize="28px">
                    Total WHALE Burned:
                  </Heading>
                  <Spacer flex={1} />
                  <Box flex={6} minWidth="100px" minHeight="100%">
                    <Heading
                      color="brandGreen"
                      fontSize="30px"
                      textAlign="right">
                      {totalWhaleBurned != null
                        ? totalWhaleBurned.toFixed(0)
                        : "-"}
                    </Heading>
                  </Box>
                </Flex>
              </Box>
            </Flex>
        </Center>
        <Spacer />
      </Flex>
    </Background>
  );
};

export default BurnPage;
