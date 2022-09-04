import { css } from "@emotion/react";
import React from "react";

interface Props {
  placeholder?: string;
  handleOnChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

export const TextInput: React.FC<Props> = (props) => {
  const handleOnChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    props.handleOnChange(e);
  };

  return (
    <div>
      <input
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
          outline: none;
        `}
        id="text-input"
        onChange={handleOnChange}
        placeholder={props.placeholder}
      />
    </div>
  );
};
