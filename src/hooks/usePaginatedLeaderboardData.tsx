import {BurnerData} from "@/types/leaderboard-data";

import {KNOWN_ADDRESSES, kPageEntries} from "@/components/shared/constants";
import { useGetSortedEntriesWithoutFoundation} from "@/hooks/useGetAllSortedEntries";

export const usePaginatedLeaderboardData = (pageNumber: number) => {
    const offset = pageNumber * kPageEntries;

    const {data: allEntries, isLoading, isError} = useGetSortedEntriesWithoutFoundation();

    const data = new Map<number, BurnerData>();

    allEntries?.slice(offset, offset + kPageEntries).forEach((value) => {
        // @ts-ignore
        data.set(allEntries.indexOf(value) + 1, {id: KNOWN_ADDRESSES?.[value[0]] || value[0] , totalBurn: value[1]});
    });
    return {data, isLoading, isError};
}
