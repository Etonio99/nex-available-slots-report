import "./css/app.css";
import LoadingIndicator from "./components/loading-indicator";

const App = () => {
  return (
    <main className="h-screen bg-cream-50">
      <div className="h-full grid place-items-center">
        <LoadingIndicator />
      </div>
    </main>
  );
}

export default App;