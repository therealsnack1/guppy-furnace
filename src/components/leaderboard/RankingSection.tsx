import {
  Center,
  Divider,
  Box,
  Spinner,
  Text,
  VStack,
  useToast,
} from "@chakra-ui/react";
import PageSelect from "./PageSelect";
import RankingTable from "./RankingTable";
import { useState } from "react";
import { usePaginatedLeaderboardData } from "@/hooks/usePaginatedLeaderboardData";

type RankingSectionData = {
  totalPages: number;
};

const RankingSection = ({ totalPages }: RankingSectionData) => {
  const [selectedPage, setSelectedPage] = useState(0);

  const onSelect = (newSelectedPage: number) =>
    setSelectedPage(newSelectedPage);

  const { data, isLoading, isError} = usePaginatedLeaderboardData(selectedPage);

  const toast = useToast();

  if (isError) {
    const id = "leaderboard-data-error";

    if (!toast.isActive(id)) {
      toast({
        description: `Error loading leaderboard data.`,
        id: id,
        duration: 10000,
        position: "top",
        status: "error",
        isClosable: true,
      });
    }
  }

  return (
    <>
      <Box minHeight="300px">
        {!isLoading ? (
          <RankingTable data={data} />
        ) : (
          <Center>
            <VStack>
              <Box height="110px" />
              <Spinner color="brandGreen" size="xl" />
              <Text color="white" paddingTop={5}>
                Loading leaderboard data...
              </Text>
            </VStack>
          </Center>
        )}
      </Box>
      <Divider paddingTop={2} />
      <Center gap={4} paddingTop={1}>
        {!isLoading && (
          <PageSelect
            selectedPage={selectedPage}
            totalPages={totalPages}
            onSelect={onSelect}
          />
        )}
      </Center>
    </>
  );
};

export default RankingSection;
