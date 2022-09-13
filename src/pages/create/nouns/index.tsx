import { css } from "@emotion/react";
import { Button, Card, Grid, Loading, Row, Spacer } from "@nextui-org/react";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { NextPage } from "next";
import { useState } from "react";
import { icons } from "../../../components/atoms/Icon";
import { PathInput } from "../../../components/PathInput";

const CreateOfNounsByYear: NextPage = () => {
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
    await invoke("create_of_nouns_by_year", {
      csvPath,
      dictionaryPath,
      userDictionaryPath,
    })
      .then((_) => {
        setLoading(false);
      })
      .catch((_) => {
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
                  Import
                </Button>
              </Row>
            </Card.Footer>
          </Card>
        </Grid>

        {isLoading && <Loading />}
      </div>
    </>
  );
};

export default CreateOfNounsByYear;
