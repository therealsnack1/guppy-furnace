import { Window as KeplrWindow } from "@keplr-wallet/types";

declare global {
  interface Window extends KeplrWindow {
    keplr: Keplr,
    leap: Keplr,
    cosmostation: any,
    isTerraExtensionAvailable: any,
  }
}