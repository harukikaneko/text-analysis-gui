import { css } from "@emotion/react";
import { CountsByNoun } from "../types/noun";

interface Props {
  countsByNoun: CountsByNoun[];
}

export const CountsByNounTable: React.FC<Props> = (props) => (
  <table
    css={css`
      width: 100%;
      text-align: center;
      border-spacing: 0;
      border-radius: 8px;
      border: 1px solid transparent;
      color: #353535;
      background-color: #ffffff;
      box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
      outline: none;
      padding: 10px;
    `}
  >
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
