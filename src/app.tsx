import "./css/app.css";
import LoadingIndicator from "./components/loading-indicator";
import { invoke } from "@tauri-apps/api/core";
import Accordion from "./components/accordion";
import { BiHomeSmile } from "react-icons/bi";
import MultiSelect from "./components/multi-select";

const App = () => {
  const test = async () => {
    const response = await invoke("get_locations");
    console.log(response);
  }

  return (
    <main className="h-screen bg-sandstone-50 p-4">
      <div className="-z-10 pointer-events-none">
        <LoadingIndicator />
      </div>
      <Accordion icon={<BiHomeSmile />} label="Test Label" text="This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text." />
      <button onClick={test}>Test!</button>
      <MultiSelect items={[
        {
          label: "Yo",
          description: "Homies",
          key: "yo-homies",
          checked: false,
        },
        {
          label: "Yo",
          description: "Homies",
          key: "yo-homies",
          checked: false,
        },
        {
          label: "Yo",
          description: "Homies",
          key: "yo-homies",
          checked: false,
        },
        {
          label: "Yo",
          description: "Homies",
          key: "yo-homies",
          checked: false,
        },
        {
          label: "Yo",
          description: "Homies",
          key: "yo-homies",
          checked: false,
        },
        {
          label: "Yo",
          description: "Homies",
          key: "yo-homies",
          checked: false,
        }
      ]} />
    </main>
  );
}

export default App;