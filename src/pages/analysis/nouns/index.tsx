import { css } from "@emotion/react";
import { Button, Card, Grid, Loading, Row, Spacer } from "@nextui-org/react";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { NextPage } from "next";
import { useState } from "react";
import { icons } from "../../../components/atoms/Icon";
import { CountsByNounTable } from "../../../components/CountsByNounTable";
import { PathInput } from "../../../components/PathInput";
import { CountsOfNounByYear } from "../../../types/noun";

const CountsByYear: NextPage = () => {
  const [items, setItems] = useState<CountsOfNounByYear[]>([]);
  const [csvPath, setCsvPath] = useState<string | string[] | null>("set csv");
  const [isLoading, setLoading] = useState(false);
  const [dictionaryPath, setDictionaryPath] = useState<
    string | string[] | null
  >("set dictionary");
  const [userDictionaryPath, setUserDictionaryPath] = useState<
    string | string[] | null
  >("set user dictionary");

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
          justify-content: center;
          text-align: center;
        `}
      >
        <Grid>
          <Card>
            <Card.Body>
              <PathInput text={csvPath as string} onClick={selectCsvPath}>
                {icons.file}
              </PathInput>
              <Spacer y={1} />
              <PathInput
                text={dictionaryPath as string}
                onClick={selectDictionaryPath}
              >
                {icons.file}
              </PathInput>
              <Spacer y={1} />
              <PathInput
                text={userDictionaryPath as string}
                onClick={selectUserDictionaryPath}
              >
                {icons.file}
              </PathInput>
            </Card.Body>
            <Card.Divider />
            <Card.Footer>
              <Row justify="flex-end">
                <Button
                  size="sm"
                  color="secondary"
                  onClick={counts_of_nouns_by_year}
                >
                  Analysis
                </Button>
              </Row>
            </Card.Footer>
          </Card>
        </Grid>

        {isLoading && <Loading />}
        <div
          css={css`
            display: flex;
            margin-top: 0.625rem;
            justify-content: space-evenly;
          `}
        >
          {items
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
