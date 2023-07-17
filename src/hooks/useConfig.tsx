import config from "../../public/config.json";

export const useConfig = () => {
  const chainInfo = config.chain_info;

  const leaderboardDataEndpoint = config.leaderboard_data_endpoint;

  const furnaceContractAddress = config.furnace_contract_address;

  return { chainInfo, leaderboardDataEndpoint, furnaceContractAddress };
};
