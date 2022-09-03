import { CountsByNoun } from "../types/noun";

interface Props {
  countsByNoun: CountsByNoun[];
}

export const CountsByNounTable: React.FC<Props> = (props) => (
  <table>
    <thead>
      <tr>
        <th>nouns</th>
        <th>count</th>
      </tr>
    </thead>
    <tbody>
      {props.countsByNoun.map((item, index) => (
        <tr key={index}>
          <td>{item.noun}</td>
          <td>{item.counts}</td>
        </tr>
      ))}
    </tbody>
  </table>
);
