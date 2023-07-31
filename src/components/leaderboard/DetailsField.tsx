import { Flex, Text } from "@chakra-ui/react";

type DetailsFieldProps = {
    label: string;
    data: string;
    dataColor: string;
};

const DetailsField = ({ label, data, dataColor }: DetailsFieldProps) => {
    return (
        <Flex minWidth={'250px'}>
            <Text color="lightGray" fontSize="16px" flex={5}>
                {label}
            </Text>
            <Text color={dataColor} fontSize="16px" flex={3} align="right">
                {data}
            </Text>
        </Flex>
    );
};

DetailsField.defaultProps = {
    dataColor: "white",
};

export default DetailsField;
