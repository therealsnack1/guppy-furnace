import React from "react";
import {
  Box,
  FormControl,
  FormLabel,
  NumberInput,
  HStack,
  Text,
  Image,
  Flex,
  Divider,
  NumberInputField,
  FormErrorMessage,
} from "@chakra-ui/react";

type BurnInputProps = {
  label: string;
  tokenIcon: string;
  tokenLabel: string;
  isReadOnly: boolean;
  onChange?: (valueAsString: string, valueAsNumber: number) => void;
  value?: string;
  isInvalid?: boolean;
};

const BurnInput = ({
  label,
  tokenIcon,
  tokenLabel,
  isReadOnly,
  onChange,
  value,
  isInvalid,
}: BurnInputProps) => {
  return (
    <FormControl isInvalid={isInvalid}>
      <Flex minWidth="100%" direction="column">
        <FormLabel color="white">{label}</FormLabel>
        <Box
          bg="black"
          paddingLeft={2}
          paddingRight={2}
          borderRadius="lg"
          borderWidth="1px"
          borderColor="darkGray"
        >
          <Flex>
            <Flex flex={2} alignItems="center">
              <NumberInput
                flex={1}
                focusBorderColor="transparent"
                errorBorderColor="transparent"
                isReadOnly={isReadOnly}
                keepWithinRange={true}
                clampValueOnBlur={false}
                onChange={onChange}
                value={value}
              >
                <NumberInputField textColor="white" border="hidden" />
              </NumberInput>
            </Flex>
            <HStack flex={1} paddingRight={4}>
              <Divider
                orientation="vertical"
                size="1px"
                borderColor="darkGray"
                opacity={1}
              />
              <Flex alignItems="center">
                <Box padding={2} alignItems="center" width={50}>
                  <Image src={tokenIcon} alt="" />
                </Box>
                <Text color="white" fontWeight={600}>
                  {tokenLabel}
                </Text>
              </Flex>
            </HStack>
          </Flex>
        </Box>
        <Box height="25px" color="transparent">
          <FormErrorMessage>Insufficient funds</FormErrorMessage>
        </Box>
      </Flex>
    </FormControl>
  );
};

export default BurnInput;
