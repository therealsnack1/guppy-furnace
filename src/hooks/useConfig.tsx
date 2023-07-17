import config from "../../public/config.json";

export const useConfig = () => {
  const chainInfo = config.chain_info;

  const furnaceContractAddress = config.furnace_contract_address;

  return { chainInfo, furnaceContractAddress };
};
