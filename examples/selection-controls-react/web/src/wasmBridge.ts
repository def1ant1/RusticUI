import type {
  ControlHook,
  TelemetryDelegate,
  TelemetryEvent,
} from '@pkg/selection_controls';

let wasmModulePromise: Promise<typeof import('@pkg/selection_controls')> | undefined;

// Centralized lazy loader so both React runtime code and Jest tests resolve the wasm bindings once.
export const loadWasm = async () => {
  if (!wasmModulePromise) {
    wasmModulePromise = import('@pkg/selection_controls');
  }
  return wasmModulePromise;
};

export const createDelegate = async (): Promise<TelemetryDelegate> => {
  const wasm = await loadWasm();
  return new wasm.TelemetryDelegate();
};

export const useCheckboxControlled = async (
  id: string,
  checked: boolean,
  delegate?: TelemetryDelegate,
): Promise<ControlHook> => {
  const wasm = await loadWasm();
  return wasm.use_checkbox_controlled(id, checked, delegate);
};

export const useCheckboxUncontrolled = async (
  id: string,
  defaultChecked: boolean,
  delegate?: TelemetryDelegate,
): Promise<ControlHook> => {
  const wasm = await loadWasm();
  return wasm.use_checkbox_uncontrolled(id, defaultChecked, delegate);
};

export type { TelemetryDelegate, TelemetryEvent, ControlHook } from '@pkg/selection_controls';
