import { useEffect, useState } from 'react';
import { CssBaseline, Container, Stack, Typography, FormControlLabel, Switch, Checkbox } from '@mui/material';
import { useSelectionControl } from './hooks/useSelectionControl';

export const App = () => {
  const [controlledValue, setControlledValue] = useState(false);
  const checkbox = useSelectionControl({ controlId: 'alerts-checkbox', checked: controlledValue });
  const radio = useSelectionControl({ controlId: 'daily-radio', defaultChecked: false });

  useEffect(() => {
    if (checkbox.events.length > 0) {
      // Controlled automation: update React state if wasm reports divergence.
      const latest = checkbox.events[checkbox.events.length - 1];
      if (latest.checked !== controlledValue) {
        setControlledValue(latest.checked);
      }
    }
  }, [checkbox.events, controlledValue]);

  return (
    <>
      <CssBaseline />
      <Container maxWidth="md" sx={{ py: 6 }}>
        <Typography variant="h4" component="h1" gutterBottom>
          React + Rust Selection Controls
        </Typography>
        <Typography variant="body1" paragraph>
          Each control below is powered by a Rust state machine compiled to WebAssembly. Telemetry
          events stream into the log, demonstrating how automation tooling can assert hydration and
          interaction flows end-to-end.
        </Typography>
        <Stack spacing={4} direction={{ xs: 'column', md: 'row' }}>
          <FormControlLabel
            control={
              <Checkbox
                checked={controlledValue}
                onChange={(_, next) => {
                  setControlledValue(next);
                  checkbox.setChecked(next);
                }}
                onClick={() => checkbox.toggle()}
              />
            }
            label="Receive Alerts"
          />
          <FormControlLabel
            control={<Switch defaultChecked onClick={() => radio.toggle()} />}
            label="Enable Automation"
          />
        </Stack>
        <Typography variant="h6" component="h2" sx={{ mt: 6 }}>
          Telemetry Timeline
        </Typography>
        <Typography variant="body2" color="text.secondary" paragraph>
          The timeline renders the raw events emitted by the wasm layer so observers can validate
          that lifecycle, programmatic, and user-driven transitions occur in a deterministic order.
        </Typography>
        <ul data-testid="telemetry-log">
          {checkbox.events.map((event) => (
            <li key={`${event.sequence}-${event.action}`}>
              #{event.sequence} [{event.control_kind}] {event.action} → {String(event.checked)} ({' '}
              {event.source}, controlled={String(event.controlled)})
            </li>
          ))}
        </ul>
      </Container>
    </>
  );
};
