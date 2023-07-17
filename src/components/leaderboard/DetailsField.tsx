import { Flex, Spacer, Text } from "@chakra-ui/react";

type DetailsFieldProps = {
  label: string;
  data: string;
  dataColor: string;
};

const DetailsField = ({ label, data, dataColor }: DetailsFieldProps) => {
  return (
    <Flex minWidth="100%">
      <Text color="lightGray" fontSize="16px" flex={5}>
        {label}
      </Text>
      <Spacer />
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
