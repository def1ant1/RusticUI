import { render, screen, waitFor } from '@testing-library/react';
import { App } from '../App';
import { loadWasm } from '../wasmBridge';

describe('React hydration with wasm selection controls', () => {
  beforeAll(async () => {
    await loadWasm();
  });

  it('streams lifecycle and user telemetry', async () => {
    render(<App />);
    const log = await screen.findByTestId('telemetry-log');
    await waitFor(() => {
      expect(log.querySelectorAll('li').length).toBeGreaterThan(0);
    });
    const first = log.querySelector('li');
    expect(first?.textContent).toContain('mount');
  });
});
