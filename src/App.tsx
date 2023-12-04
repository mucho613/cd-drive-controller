import { LcdDisplay } from "./feature/LcdDisplay/component/LcdDisplay";
import { ControlPanel } from "./feature/ControlPanel/component/ControlPanel";
import { TocList } from "./feature/TocList/component/TocList";
import { TocProvider } from "./tocContext";

function App() {
  return (
    <div class="flex flex-col">
      <h1 class="uppercase mx-3 my-2 font-semibold">CD Drive Controller</h1>

      <TocProvider>
        <LcdDisplay />
        <ControlPanel />
        <TocList />
      </TocProvider>
    </div>
  );
}

export default App;
