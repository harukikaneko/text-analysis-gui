import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { TextInput } from "../components/atoms/TextInput";
import { Button } from "../components/atoms/Button";
import { CountsByNounTable } from "../components/CountsByNounTable";
import { css } from "@emotion/react";

interface CountsByNoun {
  noun: string;
  counts: number;
}

const App: React.FC = () => {
  const [countsByNoun, setCountsByNoun] = useState<CountsByNoun[]>([]);
  const [text, setText] = useState("");

  const count_by_noun = async () => {
    await invoke("count_by_noun", { text })
      .then((result: CountsByNoun[]) => {
        setCountsByNoun(result.sort((a, b) => b.counts - a.counts));
      })
      .catch((err) => {
        console.error("count_by_noun", err);
      });
  };

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

      <div
        css={css`
          display: flex;
          justify-content: center;
        `}
      >
        <div>
          <TextInput
            placeholder="Enter a text..."
            handleOnChange={(e: React.ChangeEvent<HTMLInputElement>) =>
              setText(e.currentTarget.value)
            }
          />
          <Button text="Analysis" onClick={count_by_noun} />
        </div>
      </div>

      <CountsByNounTable countsByNoun={countsByNoun} />
    </div>
  );
};

export default App;
