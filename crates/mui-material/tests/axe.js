import axe from 'axe-core';

// Wrapper function exposed to wasm tests.
// It runs axe-core against the provided DOM node and returns the violations.
export async function runAxe(node) {
  const results = await axe.run(node);
  // Only return violations to keep payload small for wasm.
  return { violations: results.violations };
}
