import {
  Table,
  Thead,
  Tbody,
  Tr,
  Th,
  Td,
  TableContainer,
} from "@chakra-ui/react";
import { ReactNode, useMemo } from "react";
import { truncateAddress } from "@/utils/truncateAddress";
import { BurnerData } from "@/types/leaderboard-data";
import { kPageEntries, uwhalePerWhale } from "../shared/constants";

type RankingTableProps = {
  data: Map<number, BurnerData> | null;
};
const RankingTable = ({ data }: RankingTableProps) => {
  const getRowsToRender = (): ReactNode[] => {
    const rows: ReactNode[] = [];

    if (data != null) {
      data.forEach((value, key) => {
        rows.push(
          <Tr key={Math.random()}>
            <Td>{`#${key}`}</Td>
            <Td color="brandGreen">{truncateAddress(value.id)}</Td>
            <Td isNumeric>
              {(Number.parseInt(value.totalBurn) / uwhalePerWhale).toFixed(3)}
            </Td>
          </Tr>
        );
      });
    } else {
      Array.from({ length: kPageEntries }, ()=>{
        rows.push(
          <Tr key={Math.random()}>
            <Td>-</Td>
            <Td color="brandGreen">-</Td>
            <Td isNumeric>-</Td>
          </Tr>
        );
      })
    }

    return rows;
  };

  const rowsToRender = useMemo(getRowsToRender, [data]);

  return (
    <TableContainer paddingLeft={12} paddingRight={12}>
      <Table>
        <Thead>
          <Tr>
            <Th>Rank</Th>
            <Th>Address / Name</Th>
            <Th isNumeric>Total Burned</Th>
          </Tr>
        </Thead>
        <Tbody>{rowsToRender}</Tbody>
      </Table>
    </TableContainer>
  );
};

export default RankingTable;
