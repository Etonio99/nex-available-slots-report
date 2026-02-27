import "./css/app.css";
import LoadingIndicator from "./components/loading-indicator";
import { invoke } from "@tauri-apps/api/core";
import Accordion from "./components/accordion";
import { BiError, BiHomeSmile } from "react-icons/bi";
import MultiSelect from "./components/multi-select";
import Button from "./components/button";

const App = () => {
  const test = async () => {
    const response = await invoke("test");
    console.log(response);
  }

  return (
    <main className="h-screen bg-sandstone-50 p-4">
      <div className="-z-10 pointer-events-none">
        <LoadingIndicator />
      </div>
      <Accordion icon={<BiHomeSmile />} label="Test Label" text="This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text." />
      <Accordion icon={<BiHomeSmile />} label="Test Label" text="This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text." />
      <Accordion icon={<BiHomeSmile />} label="Test Label" text="This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text. This is a bunch of text." />
      <div className="flex gap-2 my-2">
        <Button label="Get Locations" style="primary" onClick={test} />
        <Button label="Get Locations" style="secondary" />
        <Button label="Get Locations" style="tertiary" />
        <Button label="Nice!" style="success" />
        <Button label="Continue" style="warning" icon={<BiError />} />
        <Button label="Delete" style="danger" />
      </div>
      <MultiSelect items={[
        {
          label: "Yo",
          description: "Homies",
          uniqueKey: "yo-homies1",
          checked: false,
        },
        {
          label: "Yo",
          description: "Homies",
          uniqueKey: "yo-homies2",
          checked: false,
        },
        {
          label: "Yo",
          description: "Homies",
          uniqueKey: "yo-homies3",
          checked: false,
        },
        {
          label: "Yo",
          description: "Homies",
          uniqueKey: "yo-homies4",
          checked: false,
        },
        {
          label: "Yo",
          description: "Homies",
          uniqueKey: "yo-homies5",
          checked: false,
        },
        {
          label: "Yo",
          description: "Homies",
          uniqueKey: "yo-homies6",
          checked: false,
        }
      ]} />
    </main>
  );
}

export default App;