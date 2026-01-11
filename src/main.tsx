import React from 'react';
import ReactDOM from 'react-dom/client';

import { ServicesProvider } from '@/hooks/useServices.tsx';

import App from './App';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <ServicesProvider>
      <App />
    </ServicesProvider>
  </React.StrictMode>,
);
