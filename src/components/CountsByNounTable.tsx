import { Table } from "@nextui-org/react";
import { CountsByNoun } from "../types/noun";

interface Props {
  countsByNoun: CountsByNoun[];
}

export const CountsByNounTable: React.FC<Props> = (props) => (
  <Table
    compact
    aria-label="counts by noun table"
    css={{
      height: "auto",
      minWidth: "100%",
    }}
  >
    <Table.Header>
      <Table.Column align={"center"}>nouns</Table.Column>
      <Table.Column align={"center"}>counts</Table.Column>
    </Table.Header>
    <Table.Body>
      {props.countsByNoun.map((item, index) => (
        <Table.Row key={index}>
          <Table.Cell>{item.noun}</Table.Cell>
          <Table.Cell>{item.counts}</Table.Cell>
        </Table.Row>
      ))}
    </Table.Body>
  </Table>
);
