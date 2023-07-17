import { useAshBalance } from "@/hooks/useAshBalance";
import { Center, VStack, Text, Box, Flex } from "@chakra-ui/react";

type SuccessDetailsProps = {
  whaleBurned: number;
};

const SuccessDetails = ({ whaleBurned }: SuccessDetailsProps) => {
  const ashBalance = useAshBalance();

  return (
    <Box
      background="black"
      borderRadius="20px"
      paddingTop={7}
      paddingBottom={10}
      marginTop={3}
    >
      <Center>
        <VStack minWidth="100%">
          <Text
            color="white"
            fontSize="30px"
            fontWeight={500}
            paddingBottom={2}
          >
            Success!
          </Text>
          <SuccessDetailField
            label="WHALE Burned"
            value={whaleBurned.toFixed(3)}
          />
          <SuccessDetailField
            label="ASH Received"
            value={whaleBurned.toFixed(3)}
          />
          <SuccessDetailField
            label="Current ASH Balance"
            value={(ashBalance + whaleBurned).toFixed(3)}
          />
        </VStack>
      </Center>
    </Box>
  );
};

type SuccessDetailFieldProps = {
  label: string;
  value: string;
};

const SuccessDetailField = ({ label, value }: SuccessDetailFieldProps) => {
  return (
    <Flex
      justifyContent="space-between"
      minWidth="100%"
      paddingLeft={20}
      paddingRight={20}
    >
      <Text flex={1} color="lightGray" align="left">
        {label}
      </Text>
      <Text flex={1} color="brandGreen" align="right">
        {value}
      </Text>
    </Flex>
  );
};

export default SuccessDetails;
