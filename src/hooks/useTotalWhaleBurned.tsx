import {useMemo} from "react";
import {useGetAllSortedEntries} from "@/hooks/useGetAllSortedEntries";
import {uwhalePerWhale} from "@/components/shared/constants";

export const useTotalWhaleBurned = () => {
    const {data} = useGetAllSortedEntries();

    return useMemo(() => {
        return data?.reduce((acc, value) => acc + Number.parseInt(value[1]), 0) / uwhalePerWhale ?? 0;
    }, [data]);
};
