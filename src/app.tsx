import "./css/app.css";
import LoadingIndicator from "./components/loading-indicator";
import { invoke } from "@tauri-apps/api/core";

const App = () => {
  const test = async () => {
    const response = await invoke("get_locations");
    console.log(response);
  }

  return (
    <main className="h-screen bg-cream-50">
      <LoadingIndicator />
      <button onClick={test}>Test!</button>
    </main>
  );
}

export default App;