import './App.css';
import Start from './pages/desktop/Start.tsx';

import { Provider } from '@/components/ui/provider';

function App() {
  return (
    <Provider>
      <main className="container">
        <Start />
      </main>
    </Provider>
  );
}

export default App;
