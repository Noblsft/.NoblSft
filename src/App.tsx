import './App.css';
import { HashRouter, Routes, Route } from 'react-router-dom';

import { Topbar, Provider } from '@/components';
import { Start, Home } from '@/pages';

function App() {
  return (
    <Provider>
      <main className="window-shell">
        <Topbar />
        <HashRouter>
          <Routes>
            <Route path="/" element={<Start />} />
            <Route path="/home" element={<Home />} />
          </Routes>
        </HashRouter>
      </main>
    </Provider>
  );
}

export default App;
