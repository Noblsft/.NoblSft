import './App.css';
import Start from './pages/desktop/Start.tsx';

import { Topbar, Provider } from '@/components';

function App() {
  return (
    <Provider>
      <main className="window-shell">
        <Topbar />
        <Start />
      </main>
    </Provider>
  );
}

export default App;
