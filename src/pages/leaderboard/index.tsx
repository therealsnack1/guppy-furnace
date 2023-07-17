import { Center, VStack, Box, Heading, Flex, Divider } from "@chakra-ui/react";
import Navbar from "@/components/navbar/Navbar";
import Background from "@/components/shared/Background";
import DetailsDisplay from "@/components/leaderboard/DetailsDisplay";
import DetailsField from "@/components/leaderboard/DetailsField";
import RankingSection from "@/components/leaderboard/RankingSection";
import { useUserRank } from "@/hooks/useUserRank";
import { useAshBalance } from "@/hooks/useAshBalance";
import { walletClientState } from "@/components/shared/state";
import { useRecoilValue } from "recoil";
import {kPageEntries} from "@/components/shared/constants";
import {useGetAllSortedEntries} from "@/hooks/useGetAllSortedEntries";
import {useTotalWhaleBurned} from "@/hooks/useTotalWhaleBurned";

export default function LeaderboardPage() {
  const userRankData = useUserRank();

  const {data: totalBurnersData}= useGetAllSortedEntries();

  const totalWhaleBurned = useTotalWhaleBurned();

  const ashBalance = useAshBalance();

  const walletClient = useRecoilValue(walletClientState);

  return (
    <Background>
      <VStack>
        <Navbar />
        <Center minHeight="100%">
          <Box
            width="850px"
            height="625px"
            background="darkGray"
            backdropFilter="blur(5px)"
            borderRadius="20px"
            paddingTop={10}
            paddingBottom={10}
          >
            <VStack align="left">
              <Heading
                color="white"
                fontSize="26px"
                paddingBottom="20px"
                paddingLeft={12}
                paddingRight={12}
              >
                Leaderboard
              </Heading>
              <Flex
                justifyContent="space-between"
                paddingLeft={12}
                paddingRight={12}
              >
                <DetailsDisplay>
                  <DetailsField
                    label="Total WHALE Burned"
                    data={
                      totalWhaleBurned != null
                        ? totalWhaleBurned.toFixed(3)
                        : "-"
                    }
                  />
                  <DetailsField
                    label="Total Burners"
                    data={
                      totalBurnersData != null
                        ? (totalBurnersData?.length ?? 0).toLocaleString()
                        : "-"
                    }
                  />
                </DetailsDisplay>
                <DetailsDisplay>
                  <DetailsField
                    label="My Rank"
                    data={
                      userRankData != null && userRankData > 0
                        ? `#${userRankData}`
                        : "-"
                    }
                    dataColor="brandGreen"
                  />
                  <DetailsField
                    label="My Ash Tokens"
                    data={walletClient != null ? ashBalance.toFixed(3) : "-"}
                  />
                </DetailsDisplay>
              </Flex>
              <Divider paddingTop={3} paddingBottom={2} />
              <RankingSection
                totalPages={
                  !!totalBurnersData ? Math.ceil((totalBurnersData.length) / kPageEntries) : 1
                }
              />
            </VStack>
          </Box>
        </Center>
      </VStack>
    </Background>
  );
}
