import axe from 'axe-core';

// Wrapper function exposed to wasm tests.  It runs axe-core against the
// provided DOM node and returns the violations collection for lightweight
// marshaling back into Rust.
export async function runAxe(node) {
  const results = await axe.run(node);
  return { violations: results.violations };
}
