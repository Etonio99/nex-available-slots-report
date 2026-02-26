import "./css/app.css";
import LoadingIndicator from "./components/loading-indicator";

const App = () => {
  return (
    <main className="h-screen bg-cream-50">
      <LoadingIndicator />
    </main>
  );
}

export default App;