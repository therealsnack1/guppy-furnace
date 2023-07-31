import {walletClientState} from "@/components/shared/state";
import {useRecoilValue} from "recoil";
import {useGetSortedEntriesWithoutFoundation} from "@/hooks/useGetAllSortedEntries";

export const useUserRank = () => {
    const {data} = useGetSortedEntriesWithoutFoundation()

    const walletClient = useRecoilValue(walletClientState);

    const userAddress = walletClient?.address;

    return userAddress && data ? (data.findIndex((value) => value[0] === userAddress) + 1) : null;

};
