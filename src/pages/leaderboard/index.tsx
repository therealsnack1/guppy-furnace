import {Center, VStack, Box, Heading, Flex, Divider} from "@chakra-ui/react";
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
import {useWhaleBurned} from "@/hooks/useWhaleBurned";

export default function LeaderboardPage() {
  const userRankData = useUserRank();

  const {data: totalBurnersData} = useGetAllSortedEntries();

  const {communityBurn, foundationBurn, totalBurned}= useWhaleBurned();

  const ashBalance = useAshBalance();

  const walletClient = useRecoilValue(walletClientState);

  return (
      <Background>
        <VStack>
          <Navbar />
          <Center minHeight="100%">
            <Box
                width="950px"
                height="643px"
                background={"darkGray"}
                backdropFilter="blur(5px)"
                borderRadius="20px"
                mt={-5}
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
                    flexDirection="column"
                    paddingLeft={12}
                    paddingRight={12}
                >
                  <DetailsDisplay>
                    <DetailsField
                        label="Total Burned"
                        data={
                          totalBurned != null && !isNaN(totalBurned)
                              ? totalBurned.toLocaleString()
                              : "-"
                        }
                    />
                    <DetailsField
                        label="Community Burn"
                        data={
                          communityBurn != null && !isNaN(communityBurn)
                              ? communityBurn.toLocaleString()
                              : "-"
                        }
                    />
                    <DetailsField
                        label="Foundation Burn"
                        data={
                          foundationBurn != null && !isNaN(foundationBurn)
                              ? foundationBurn.toLocaleString()
                              : "-"
                        }
                    />
                  </DetailsDisplay>
                  <Box height={5} />
                  <DetailsDisplay>
                    <DetailsField
                        label="Total Burners"
                        data={
                          totalBurnersData != null
                              ? (totalBurnersData?.length ?? 0).toLocaleString()
                              : "-"
                        }
                    />
                    <DetailsField
                        label="My Rank"
                        data={
                          userRankData != null && userRankData > 0
                              ? `#${userRankData}`
                              : "-"
                        }
                        dataColor="brandBlue"
                    />
                    <DetailsField
                        label="My Guppy Tokens"
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
