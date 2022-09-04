import { css } from "@emotion/react";

interface Props {
  onClick: () => Promise<void>;
  text: string;
}

export const Button: React.FC<Props> = (props) => (
  <button
    css={css`
      border-radius: 8px;
      border: 1px solid transparent;
      padding: 0.6em 1.2em;
      font-size: 1em;
      font-weight: 500;
      font-family: inherit;
      color: #0f0f0f;
      background-color: #ffffff;
      transition: border-color 0.25s;
      box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
      cursor: pointer;
      &:hover {
        border-color: #396cd8;
      }
      outline: none;
    `}
    type="button"
    onClick={() => props.onClick()}
  >
    {props.text}
  </button>
);
