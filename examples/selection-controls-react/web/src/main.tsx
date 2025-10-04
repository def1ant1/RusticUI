import React from 'react';
import ReactDOM from 'react-dom/client';
import { App } from './App';
import { loadWasm } from './wasmBridge';

const container = document.getElementById('root');
if (!container) {
  throw new Error('Root container missing');
}

// Hydrate the wasm module before rendering the React tree so initial telemetry events (mount) are
// captured synchronously for automation pipelines.
loadWasm().then(() => {
  const root = ReactDOM.createRoot(container);
  root.render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  );
});
