import { useEffect, useMemo, useState } from 'react';
import type { TelemetryEvent } from '../wasmBridge';
import {
  createDelegate,
  useCheckboxControlled,
  useCheckboxUncontrolled,
  type ControlHook,
  type TelemetryDelegate,
} from '../wasmBridge';

const fallbackDelegate = (() => {
  const delegate = {
    bind: () => undefined,
    drain: () => [],
    clone_handle: () => delegate,
  } as unknown as TelemetryDelegate;
  return delegate;
})();

export interface UseSelectionControlOptions {
  controlId: string;
  checked?: boolean;
  defaultChecked?: boolean;
  telemetry?: TelemetryDelegate;
}

export interface SelectionControlApi {
  checked: boolean;
  toggle(): void;
  setChecked(next: boolean): void;
  delegate: TelemetryDelegate;
  events: TelemetryEvent[];
}

export const useSelectionControl = (
  options: UseSelectionControlOptions,
): SelectionControlApi => {
  const { controlId, checked, defaultChecked = false } = options;
  const [telemetry, setTelemetry] = useState<TelemetryEvent[]>([]);
  const [delegate, setDelegate] = useState<TelemetryDelegate | null>(options.telemetry ?? null);
  const [hook, setHook] = useState<ControlHook | null>(null);

  useEffect(() => {
    let mounted = true;
    const connect = async () => {
      const existingDelegate = delegate ?? (await createDelegate());
      const backlog = existingDelegate.drain();
      if (mounted) {
        setTelemetry(backlog);
      }
      existingDelegate.bind((event) => {
        if (!mounted) return;
        setTelemetry((previous) => [...previous, event]);
      });
      const wasmHook =
        checked !== undefined
          ? await useCheckboxControlled(controlId, checked, existingDelegate)
          : await useCheckboxUncontrolled(controlId, defaultChecked, existingDelegate);
      if (!mounted) {
        return;
      }
      if (!delegate) {
        setDelegate(existingDelegate);
      }
      setHook(wasmHook);
    };
    void connect();
    return () => {
      mounted = false;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [controlId]);

  useEffect(() => {
    if (checked !== undefined && hook) {
      hook.setChecked(checked);
    }
  }, [checked, hook]);

  const api = useMemo<SelectionControlApi>(() => {
    const activeDelegate = delegate ?? fallbackDelegate;
    return {
      checked: hook?.checked() ?? checked ?? defaultChecked,
      toggle: () => hook?.userToggle(),
      setChecked: (next: boolean) => hook?.setChecked(next),
      delegate: activeDelegate,
      events: telemetry,
    };
  }, [hook, telemetry, delegate, checked, defaultChecked]);

  return api;
};
