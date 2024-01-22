import { MenuItem } from "./components/MenuItem";
import { Separator } from "./components/Separator";

function App() {
  return (
    <div className="text-white">
      <MenuItem title="Chang" />
      <Separator />
      <MenuItem title="Settings" />
      <Separator />
      <MenuItem title="Gigantic" />
      <Separator />
      <MenuItem title="Text" />
      <Separator />
      <MenuItem title="Text" />
      <Separator />
      <MenuItem title="Text" />
    </div>
  );
}

export default App;
