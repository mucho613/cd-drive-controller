import { LcdDisplay } from "./feature/LcdDisplay/component/LcdDisplay";
import { ControlPanel } from "./feature/ControlPanel/component/ControlPanel";
import { TocList } from "./feature/TocList/component/TocList";
import { TocProvider } from "./useTocContext";

function App() {
  return (
    <div class="flex flex-col items-center">
      <h1 class="uppercase">CD Drive Controller</h1>

      <TocProvider>
        <LcdDisplay />
        <ControlPanel />
        <TocList />
      </TocProvider>
    </div>
  );
}

export default App;
