import '@testing-library/jest-dom';

// Jest runs in Node and therefore cannot natively import wasm without a loader.  The tests rely on
// the wasm-bindgen generated TypeScript bindings in `web/pkg`, so we ensure `fetch` is present and
// reads from the in-memory file system provided by Jest.
if (typeof window !== 'undefined' && !('fetch' in window)) {
  (window as unknown as { fetch: typeof fetch }).fetch = async (input: RequestInfo | URL) => {
    const fs = await import('node:fs/promises');
    const path = await import('node:path');
    const filePath = path.resolve(__dirname, String(input));
    const data = await fs.readFile(filePath);
    return new Response(data);
  };
}
