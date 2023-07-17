import {BurnerData} from "@/types/leaderboard-data";

import {kPageEntries} from "@/components/shared/constants";
import {useGetAllSortedEntries} from "@/hooks/useGetAllSortedEntries";

export const usePaginatedLeaderboardData = (pageNumber: number) => {
    const offset = pageNumber * kPageEntries;

    const {data: allEntries, isLoading, isError} = useGetAllSortedEntries();

    const data = new Map<number, BurnerData>();

    allEntries?.slice(offset, offset + kPageEntries).forEach((value) => {
        data.set(allEntries.indexOf(value) + 1, {id: value[0], totalBurn: value[1]});
    });
    return {data, isLoading, isError};
}
