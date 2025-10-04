declare module '@pkg/selection_controls' {
  export interface TelemetryEvent {
    sequence: number;
    control_kind: 'checkbox' | 'switch' | 'radio';
    control_id: string;
    action: string;
    checked: boolean;
    timestamp_ms: number;
    source: 'user' | 'programmatic' | 'lifecycle';
    controlled: boolean;
  }

  export class TelemetryDelegate {
    constructor();
    bind(callback: (event: TelemetryEvent) => void): void;
    drain(): TelemetryEvent[];
    clone_handle(): TelemetryDelegate;
  }

  export class ControlHook {
    checked(): boolean;
    userToggle(): boolean;
    setChecked(checked: boolean): boolean;
    delegate(): TelemetryDelegate;
  }

  export function use_checkbox_controlled(
    controlId: string,
    checked: boolean,
    delegate?: TelemetryDelegate
  ): ControlHook;
  export function use_checkbox_uncontrolled(
    controlId: string,
    defaultChecked: boolean,
    delegate?: TelemetryDelegate
  ): ControlHook;
  export function use_switch_controlled(
    controlId: string,
    checked: boolean,
    delegate?: TelemetryDelegate
  ): ControlHook;
  export function use_switch_uncontrolled(
    controlId: string,
    defaultChecked: boolean,
    delegate?: TelemetryDelegate
  ): ControlHook;
  export function use_radio_controlled(
    controlId: string,
    checked: boolean,
    delegate?: TelemetryDelegate
  ): ControlHook;
  export function use_radio_uncontrolled(
    controlId: string,
    defaultChecked: boolean,
    delegate?: TelemetryDelegate
  ): ControlHook;
}
