import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App.js';
import { Providers } from './components/index.js';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <Providers>
      <App />
    </Providers>
  </React.StrictMode>
);
