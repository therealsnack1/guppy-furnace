import {useMemo} from "react";
import {useGetAllSortedEntries, useGetSortedEntriesWithoutFoundation} from "@/hooks/useGetAllSortedEntries";
import {uwhalePerWhale} from "@/components/shared/constants";

export const useWhaleBurned = () => {
    const {data: allEntries} = useGetAllSortedEntries();
    const {data: entriesWithoutFoundation} = useGetSortedEntriesWithoutFoundation();

    const totalBurned = useMemo(() => allEntries?.reduce((acc, value) =>  acc + Number.parseInt(value[1]), 0) / uwhalePerWhale ?? 0, [allEntries]);
    const communityBurn = useMemo(() => entriesWithoutFoundation?.reduce((acc, value) =>  acc + Number.parseInt(value[1]), 0) / uwhalePerWhale ?? 0, [entriesWithoutFoundation]);
    const foundationBurn = totalBurned - communityBurn;

    return {totalBurned, communityBurn, foundationBurn};
};
