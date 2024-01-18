import { uwhalePerWhale } from "@/components/shared/constants";
import { ExecuteResult, SigningCosmWasmClient, } from "@cosmjs/cosmwasm-stargate";
import { BankExtension, coins, GasPrice } from "@cosmjs/stargate";
import { LCDClient, MsgExecuteContract } from "@terra-money/feather.js";
import { ConnectedWallet, TxResult } from "@terra-money/wallet-provider";
import { ChainInfo } from "@keplr-wallet/types";
const tx_3 = require("cosmjs-types/cosmos/tx/v1beta1/tx");
const tx_4 = require("cosmjs-types/cosmwasm/wasm/v1/tx");
const encoding_1 = require("@cosmjs/encoding");
const stargate_1 = require("@cosmjs/stargate");

export enum WalletClientType {
  terrastation = 'Terra Station',
  keplr = 'Keplr',
  cosmostation = 'Cosmostation',
  leap = 'Leap',
}

export abstract class WalletClient {
  public abstract type: WalletClientType;
  public abstract address: string;
  public abstract furnaceContractAddress: string;
  public abstract chainInfo: ChainInfo;

  public get ashDenom() { return `factory/migaloo1r9x8fz4alekzr78k42rpmr9unpa7egsldpqeynmwl2nfvzexue9sn8l5rg/gash` };

  abstract getBalance(denom: String): Promise<number>

  public async getWhaleBalance(): Promise<number> {
    const uwhaleBalance = await this.getBalance("factory/migaloo1etlu2h30tjvv8rfa4fwdc43c92f6ul5w9acxzk/uguppy");
    return uwhaleBalance / uwhalePerWhale;
  }

  public async getWhaleRealBalance(): Promise<number> {
    const uwhaleBalance = await this.getBalance("uwhale");
    return uwhaleBalance / uwhalePerWhale;
  }

  public async getAshBalance(): Promise<number> {
    const minimalDenomAsh = await this.getBalance(this.ashDenom);

    return minimalDenomAsh / uwhalePerWhale;
  }

  public async getTotalWhaleBurned(): Promise<number> {
    const minimalDenomAsh = await this.bank(this.ashDenom);

    return minimalDenomAsh / uwhalePerWhale;
  }

  abstract disconnect(): Promise<void>

  abstract burn(
    amount: number
  ): Promise<ExecuteResult | TxResult>

  abstract bank(denom: string): Promise<number>
}

export class CosmWasmWalletClient extends WalletClient {
  constructor(public client: SigningCosmWasmClient, public bankExtension: BankExtension, public type: WalletClientType, public address: string, public chainInfo: ChainInfo, public furnaceContractAddress: string) {
    super();
  }

  public getBalance = async (denom: string) => {
    try {
      const balance = await this.client.getBalance(this.address, denom);
      return Number.parseFloat(balance.amount);
    } catch (e) {
      console.log(`Error getting ${denom} balance: ${e}`);
      return 0;
    }
  }

  public disconnect = async (): Promise<void> => {
    this.client.disconnect();
  }

  public async burn(
    amount: number
  ) {
    // Using this.client.execute results in a TypeError originating in the cosmwasm-stargate code
    // when it tries to get and then add the property chainId to the CosmWasmClient to construct
    // the signer data. Passing in the signer data directly bypasses this error. 

    const funds = coins(amount * uwhalePerWhale, "factory/migaloo1etlu2h30tjvv8rfa4fwdc43c92f6ul5w9acxzk/uguppy");

    const burnMsg = { burn: {} };

    const storeCodeMsg = {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: tx_4.MsgExecuteContract.fromPartial({
        sender: this.address,
        contract: this.furnaceContractAddress,
        msg: (0, encoding_1.toUtf8)(JSON.stringify(burnMsg)),
        funds: funds,
      })
    }

    const gasEstimation = await this.client.simulate(this.address, [storeCodeMsg], "");
    const fee = (0, stargate_1.calculateFee)(Math.round(gasEstimation * 1.3), GasPrice.fromString(
      `${this.chainInfo.feeCurrencies[0].gasPriceStep?.low ?? 0}uwhale`
    ));

    const { accountNumber, sequence } = await this.client.getSequence(this.address);

    const signerData = {
      accountNumber: accountNumber,
      sequence: sequence,
      chainId: this.chainInfo.chainId,
    }

    const txRaw = await this.client.sign(this.address, [storeCodeMsg], fee, "", signerData);

    const txBytes = tx_3.TxRaw.encode(txRaw).finish();

    const result = await this.client.broadcastTx(txBytes, this.client.broadcastTimeoutMs, this.client.broadcastPollIntervalMs);

    return {
      logs: stargate_1.logs.parseRawLog(result.rawLog),
      height: result.height,
      transactionHash: result.transactionHash,
      events: result.events,
      gasWanted: result.gasWanted,
      gasUsed: result.gasUsed,
    };
  }

  public async bank(denom: string) {
    return Number.parseInt((await this.bankExtension.bank.supplyOf(denom)).amount);
  }
}

export class TerraStationWalletClient extends WalletClient {
  public type = WalletClientType.terrastation;
  public disconnect: () => Promise<void>;

  constructor(
    public lcdClient: LCDClient,
    public connectedWallet: ConnectedWallet,
    disconnect: () => void,
    public address: string,
    public chainInfo: ChainInfo, public furnaceContractAddress: string,
  ) {
    super();
    this.disconnect = async () => { disconnect };
  }

  public getBalance = async (denom: string) => {
    try {
      const coins = await this.lcdClient.bank.balance(this.address);

      const coin = coins[0].get(denom);

      return coin!.amount.toNumber();
    } catch (e) {
      console.log(`Error getting ${denom} balance: ${e}`);
      return 0;
    }
  }

  // This method hasn't been tested yet because Terra Station won't allow connecting to local migaloo.
  async burn(amount: number): Promise<TxResult> {

    const executeMsg = new MsgExecuteContract(
      this.address,
      this.furnaceContractAddress,
      { burn: {} },
      { uwhale: amount * uwhalePerWhale },
    );

    const transactionMsg = {
      chainID: this.chainInfo.chainId,
      msgs: [executeMsg],
    };

    const txResult = await this.connectedWallet.post(transactionMsg);

    return txResult;
  }

  async bank(denom: string): Promise<number> {
    const total = await this.lcdClient.bank.total(this.chainInfo.chainId);

    const denomCoin = total[0].get(denom);

    if (denomCoin !== undefined) {
      return denomCoin.amount.toNumber();
    } else {
      return 0;
    }
  }
}