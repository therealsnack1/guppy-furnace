import { atom } from "recoil";
import { WalletClient } from "@/types/wallet-client";


export const walletClientState = atom<WalletClient | null>({
  key: "walletClientState",
  default: null,
})

export enum BurningState {
  userInput,
  burning,
  success,
}


export const burningState = atom<BurningState>({
  key: "burningState",
  default: BurningState.userInput,
})

export const whaleBalanceState = atom<number>({
  key: "whaleBalanceState",
  default: 0,
})

export const realwhaleBalanceState = atom<number>({
  key: "realwhaleBalanceState",
  default: 0,
})

export const ashBalanceState = atom<number>({
  key: "ashBalanceState",
  default: 0,
})
