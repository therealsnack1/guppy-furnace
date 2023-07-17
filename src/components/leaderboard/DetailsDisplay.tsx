import { Box, StackDivider, VStack } from "@chakra-ui/react";
import { ReactNode } from "react";

type DetailsDisplayProps = {
  children: ReactNode;
};

const DetailsDisplay = ({ children }: DetailsDisplayProps) => {
  return (
    <Box bg="darkTransparentGray" borderRadius="15px" padding={4} width="340px">
      <VStack divider={<StackDivider borderColor="lightTransparentGray" />}>
        {children}
      </VStack>
    </Box>
  );
};

export default DetailsDisplay;
