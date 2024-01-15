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
import { useWhaleBurned } from "@/hooks/useWhaleBurned";
import { useWhaleSupply } from "@/hooks/useWhaleSupply";

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
  const {totalBurned} = useWhaleBurned();
  const totalWhaleSupply = useWhaleSupply();

  return (
      <Background>
        <Flex direction="column" height="100vh">
          <Navbar />
          <Spacer />
          <Center>
            <EllipticalShadow>
              <VStack flex={4}>
                <Text color="brandBlue" fontWeight={500} fontSize={20}>
                  GUPPY Burner
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
                  width="630px"
                  background={"rgba(0, 0, 0, 0.8)"}
                  padding={5}
                  borderRadius="20px">
                <Flex>
                  <Heading flex={13} color="white" fontSize="30px">
                    Total GUPPY Burned:
                  </Heading>
                  <Spacer />
                  <Box flex={6} minWidth="230px" minHeight="100%" >
                    <Heading
                        color="brandGreen"
                        fontSize="30px"
                        textAlign="end">
                      {totalBurned != null && !isNaN(totalBurned)
                          ? totalBurned.toLocaleString()
                          : "-"}
                    </Heading>
                  </Box>
                </Flex>
                <Flex>
                  <Heading flex={13} color="white" fontSize="30px">
                    GUPPY Supply:
                  </Heading>
                  <Spacer />
                  <Box flex={6} minWidth="230px" minHeight="100%">
                    <Heading
                        color="brandGreen"
                        fontSize="30px"
                        textAlign="end">
                      {totalWhaleSupply != null
                          ? totalWhaleSupply.toLocaleString()
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
