import { Box, Divider, HStack } from "@chakra-ui/react";
import { ReactNode } from "react";

type DetailsDisplayProps = {
    children: ReactNode;
};

const DetailsDisplay = ({ children }: DetailsDisplayProps) => {
    return (
        <Box bg={"darkTransparentGray"} borderRadius="15px" padding={3}>
            <HStack divider={<Divider orientation="vertical" borderColor="grey" h={5} />} spacing={4} width="300px">
                {children}
            </HStack>
        </Box>
    );
};

export default DetailsDisplay;
