import './css/app.css';
import { useRouter } from './hooks/useRouter';
import Home from './pages/home';
import FAQ from './pages/faq';
import Process from './pages/process';

const App = () => {
  const { page, navigate } = useRouter();

  const getPage = (pageName: string) => {
    switch (pageName) {
      case 'home':
        return <Home navigate={navigate} />;
      case 'faq':
        return <FAQ />;
      case 'process':
        return <Process />;
    }
    return null;
  };

  return (
    <main className="h-screen p-4 overflow-hidden w-screen">
      {getPage(page)}
    </main>
  );
};

export default App;
