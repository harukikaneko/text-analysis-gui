import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { TextInput } from "../components/atoms/TextInput";
import { Button } from "../components/atoms/Button";
import { CountsByNounTable } from "../components/CountsByNounTable";

interface CountsByNoun {
  noun: string;
  counts: number;
}

const App: React.FC = () => {
  const [counyByNoun, setcounyByNoun] = useState<CountsByNoun[]>([]);
  const [text, setText] = useState("");

  const count_by_noun = async () => {
    await invoke("count_by_noun", { text })
      .then((result: CountsByNoun[]) => {
        setcounyByNoun(result.sort((a, b) => b.counts - a.counts));
      })
      .catch((err) => {
        console.error("count_by_noun", err);
      });
  };

  return (
    <div className="container">
      <h1>Lets Text Analysis</h1>

      <div className="row">
        <div>
          <TextInput
            palceholder="Enter a text..."
            handleOnChange={(e: React.ChangeEvent<HTMLInputElement>) =>
              setText(e.currentTarget.value)
            }
          />
          <Button text="Analysis" onClick={count_by_noun} />
        </div>
      </div>

      <CountsByNounTable countsByNoun={counyByNoun} />
    </div>
  );
};

export default App;
