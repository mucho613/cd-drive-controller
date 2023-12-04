import { Accessor, JSX, Setter, createContext, createSignal, useContext } from "solid-js";
import { Toc } from "./feature/TocList/types/toc";

const TocContext = createContext<[Accessor<Toc | null>, Setter<Toc | null>]>();

type Props = {
  children: JSX.Element;
};

export function TocProvider(props: Props) {
  const [toc, setToc] = createSignal<Toc | null>(null);

  return (
    <TocContext.Provider value={[toc, setToc]}>{props.children}</TocContext.Provider>
  );
}

export function useToc() {
  return useContext(TocContext);
}
