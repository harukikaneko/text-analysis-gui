import { css } from "@emotion/react";
import {
  Button,
  Card,
  Grid,
  Input,
  Loading,
  Row,
  Spacer,
} from "@nextui-org/react";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { CountsByNounTable } from "../components/CountsByNounTable";
import { CountsByNoun } from "../types/noun";
import { NextPage } from "next";
import { open } from "@tauri-apps/api/dialog";
import { PathInput } from "../components/PathInput";
import { icons } from "../components/atoms/Icon";

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
        justify-content: center;
        text-align: center;
      `}
    >
      <Grid>
        <Card>
          <Card.Body>
            <Input
              clearable
              bordered
              labelPlaceholder="input text"
              onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                setText(e.currentTarget.value)
              }
            />
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
              <Button size="sm" color="secondary" onClick={count_by_noun}>
                Analysis
              </Button>
            </Row>
          </Card.Footer>
        </Card>
      </Grid>

      {isLoading && <Loading />}
      <CountsByNounTable countsByNoun={countsByNoun} />
    </div>
  );
};

export default Nouns;
