import { css } from "@emotion/react";
import { NextPage } from "next";

interface CountsByNoun {
  noun: string;
  counts: number;
}

const App: NextPage = () => {

  return (
    <div
      css={css`
        margin: 0;
        padding-top: 10vh;
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
      `}
    >
      <h1
        css={css`
          text-align: center;
        `}
      >
        Lets Text Analysis
      </h1>
    </div>
  );
};

export default App;
