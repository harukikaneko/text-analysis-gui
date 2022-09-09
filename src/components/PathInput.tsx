import { Input } from "@nextui-org/react";
import { OpenButton } from "./atoms/OpenButton";

interface Props {
  text: string;
  onClick: () => Promise<void> | void;
  children: React.ReactElement;
}

export const PathInput: React.FC<Props> = (props) => {
  return (
    <Input
      readOnly
      clearable
      contentRightStyling={false}
      placeholder={props.text}
      contentRight={
        <OpenButton onClick={() => props.onClick()}>
          {props.children}
        </OpenButton>
      }
    />
  );
};
