import { css } from "@emotion/react";
import { Loading } from "@nextui-org/react";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { Button } from "../../../components/atoms/Button";
import { TextInput } from "../../../components/atoms/TextInput";
import { CountsByNounTable } from "../../../components/CountsByNounTable";
import { CountsByNoun } from "../../../types/noun";
import { NextPage } from "next";
import { open } from "@tauri-apps/api/dialog";

const Nouns: NextPage = () => {
  const [countsByNoun, setCountsByNoun] = useState<CountsByNoun[]>([]);
  const [text, setText] = useState("");
  const [isLoading, setLoading] = useState(false);
  const [dictionaryPath, setDictionaryPath] = useState<
    string | string[] | null
  >(null);
  const [userDictionaryPath, setUserDictionaryPath] = useState<
    string | string[] | null
  >(null);

  const count_by_noun = async () => {
    setLoading(true);
    await invoke<CountsByNoun[]>("counts_by_noun", {
      text,
      dictionaryPath,
      userDictionaryPath,
    })
      .then((result) => {
        setCountsByNoun(result.sort((a, b) => b.counts - a.counts));
        setLoading(false);
      })
      .catch((err) => {
        console.error("counts_by_noun", err);
        setLoading(false);
      });
  };

  const selectDictionaryPath = async () => {
    const path = await open({ directory: true });
    setDictionaryPath(path);
  };

  const selectUserDictionaryPath = async () => {
    const path = await open({
      filters: [{ name: "Csv", extensions: ["csv"] }],
    });
    setUserDictionaryPath(path);
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
          <Button text="Set Dictionary" onClick={selectDictionaryPath} />
          <Button
            text="Set UserDictionary"
            onClick={selectUserDictionaryPath}
          />
        </div>
      </div>

      {isLoading && <Loading />}
      <CountsByNounTable countsByNoun={countsByNoun} />
    </div>
  );
};

export default Nouns;
