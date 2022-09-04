import { css } from "@emotion/react";
import { Loading } from "@nextui-org/react";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { Button } from "../../components/atoms/Button";
import { TextInput } from "../../components/atoms/TextInput";
import { CountsByNounTable } from "../../components/CountsByNounTable";
import { CountsByNoun } from "../../types/noun";

const Nouns: React.FC = () => {
  const [countsByNoun, setCountsByNoun] = useState<CountsByNoun[]>([]);
  const [text, setText] = useState("");
  const [isLoading, setLoading] = useState(false);

  const count_by_noun = async () => {
    setLoading(true);
    await invoke("count_by_noun", { text })
      .then((result: CountsByNoun[]) => {
        setCountsByNoun(result.sort((a, b) => b.counts - a.counts));
        setLoading(false);
      })
      .catch((err) => {
        console.error("count_by_noun", err);
        setLoading(false);
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
        Lets Counts by Noun
      </h1>

      <div
        css={css`
          display: flex;
          justify-content: center;
        `}
      >
        <div
          css={css`
            display: flex;
          `}
        >
          <TextInput
            placeholder="Enter a text..."
            handleOnChange={(e: React.ChangeEvent<HTMLInputElement>) =>
              setText(e.currentTarget.value)
            }
          />
          <Button text="Analysis" onClick={count_by_noun} />
        </div>
      </div>

      {isLoading && <Loading />}
      <CountsByNounTable countsByNoun={countsByNoun} />
    </div>
  );
};

export default Nouns;
