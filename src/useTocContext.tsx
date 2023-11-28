import { JSX, createContext, createSignal, useContext } from "solid-js";
import { Toc } from "./feature/TocList/types/toc";

const TocContext = createContext();

type Props = {
  children: JSX.Element;
};

export function TocProvider(props: Props) {
  const [toc, setToc] = createSignal<Toc | null>(null);

  const store = [
    toc,
    {
      setToc(toc: Toc) {
        setToc(toc);
      },
    },
  ];

  return (
    <TocContext.Provider value={store}>{props.children}</TocContext.Provider>
  );
}

export function useToc() {
  return useContext(TocContext);
}
