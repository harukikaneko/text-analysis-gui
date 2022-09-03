interface Props {
  onClick: () => Promise<void>;
  text: string;
}

export const Button: React.FC<Props> = (props) => (
  <button type="button" onClick={() => props.onClick()}>
    {props.text}
  </button>
);
