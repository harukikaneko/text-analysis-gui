import { css } from "@emotion/react";
import { Loading } from "@nextui-org/react";
import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import { NextPage } from "next";
import { useState } from "react";
import { Button } from "../../../components/atoms/Button";
import { CountsByNounTable } from "../../../components/CountsByNounTable";
import { CountsOfNounByYear } from "../../../types/noun";

const CountsByYear: NextPage = () => {
  const [items, setItems] = useState<CountsOfNounByYear[]>([]);
  const [csvPath, setCsvPath] = useState<string | string[] | null>("");
  const [isLoading, setLoading] = useState(false);
  const [dictionaryPath, setDictionaryPath] = useState<
    string | string[] | null
  >(null);
  const [userDictionaryPath, setUserDictionaryPath] = useState<
    string | string[] | null
  >(null);

  const counts_of_nouns_by_year = async () => {
    setLoading(true);
    await invoke<CountsOfNounByYear[]>("counts_of_nouns_by_year", {
      csvPath,
      dictionaryPath,
      userDictionaryPath,
    })
      .then((result) => {
        setItems(result);
        setLoading(false);
      })
      .catch((err) => {
        console.error("counts_of_nouns_by_year", err);
        setLoading(false);
      });
  };

  const selectCsvPath = async () => {
    const path = await open({
      filters: [{ name: "Csv", extensions: ["csv"] }],
    });
    setCsvPath(path);
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
    <>
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
          Lets Counts of Noun by Year
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
            <Button text="Analysis" onClick={counts_of_nouns_by_year} />
            <Button text="Set Csv" onClick={selectCsvPath} />
            <Button text="Set Dictionary" onClick={selectDictionaryPath} />
            <Button
              text="Set UserDictionary"
              onClick={selectUserDictionaryPath}
            />
          </div>
        </div>

        {isLoading && <Loading />}
        <div
          css={css`
            display: flex;
            justify-content: space-evenly;
          `}
        >
          {items.length !== 0 &&
            items
              .sort((a, b) => a.year - b.year)
              .map((item, index) => (
                <>
                  <div key={index}>
                    <p>{item.year}</p>
                    <CountsByNounTable
                      countsByNoun={item.nouns.sort(
                        (a, b) => b.counts - a.counts
                      )}
                    />
                  </div>
                </>
              ))}
        </div>
      </div>
    </>
  );
};

export default CountsByYear;
