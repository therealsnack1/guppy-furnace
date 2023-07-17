import {ExecuteEvent, SumBurn} from "../types";
import {
  CosmosEvent,
  CosmosBlock,
  CosmosMessage,
  CosmosTransaction,
} from "@subql/types-cosmos";




export async function handleEvent(event: CosmosEvent): Promise<void> {
  const eventRecord = ExecuteEvent.create({
    id: `${event.tx.hash}-${event.msg.idx}-${event.idx}`,
    blockHeight: BigInt(event.block.block.header.height),
    txHash: event.tx.hash,
    // @ts-ignore
    contractAddress: event.event.attributes.find(attr => attr.key === '_contract_address').value,
    sender: event.msg.msg.decodedMsg.sender,
    // @ts-ignore
    burnAmount: BigInt(event.msg.msg.decodedMsg.funds[0].amount),
  });
  await eventRecord.save();
}

function createSumRecord(accountId: string): SumBurn {
  // @ts-ignore
  const entity = new SumBurn(accountId);
  entity.totalBurn = BigInt(0);
  return entity;
}

export async function handleSumBurn(event: CosmosEvent): Promise<void> {

  const sender = event.msg.msg.decodedMsg.sender;
  let entity = await SumBurn.get(sender.toString());

  if (entity === undefined){
    entity = createSumRecord(sender.toString());
  }
  entity.totalBurn = entity.totalBurn + BigInt(event.msg.msg.decodedMsg.funds[0].amount);
  entity.blockheight = BigInt(event.block.block.header.height)

  await entity.save();
}
