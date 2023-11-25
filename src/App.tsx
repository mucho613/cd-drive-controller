import { LcdDisplay } from "./feature/LcdDisplay/component/LcdDisplay";
import { ControlPanel } from "./feature/ControlPanel/component/ControlPanel";

function App() {
  return (
    <div class="flex flex-col items-center">
      <h1 class="uppercase">CD Drive Controller</h1>
      
      <LcdDisplay />
      <ControlPanel />
      {/* <TocList toc={toc()}/> */}
    </div>
  );
}

export default App;
