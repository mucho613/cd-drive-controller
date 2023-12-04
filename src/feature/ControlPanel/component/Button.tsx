type Props = {
  label: string;
  onClick: () => void;
};

export function Button(props: Props) {
  return (
    <button type="button" class="border rounded w-10 h-10">
      {props.label}
    </button>
  );
}
