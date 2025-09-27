import path from 'path';
import { resolvePackageSourceRoot, rustDocFlags } from './rustDocAutomation.js';

/**
 * These project definitions intentionally point at the archived npm packages
 * (under ./archives/mui-packages) so local contributors can continue to run
 * the TypeScript-based generators. CI can flip the Rust flags to instead read
 * from the metadata emitted by the Rust crates.
 */
const materialRoot = resolvePackageSourceRoot('mui-material');
const labRoot = resolvePackageSourceRoot('mui-lab');
const joyRoot = resolvePackageSourceRoot('mui-joy');
const systemRoot = resolvePackageSourceRoot('mui-system');

if (rustDocFlags.shouldSkipArchives) {
  console.log(
    'ℹ️  RUSTIC_UI_DOCS_RUST_AUTHORITATIVE detected – skipping archived TypeScript packages in favour of Rust outputs where available.',
  );
}

export default {
  material: {
    rootPath: materialRoot,
    entryPointPath: 'src/index.d.ts',
  },
  lab: {
    rootPath: labRoot,
    entryPointPath: 'src/index.d.ts',
  },
  joy: {
    rootPath: joyRoot,
    entryPointPath: 'src/index.ts',
  },
  system: {
    rootPath: systemRoot,
    entryPointPath: 'src/index.d.ts',
  },
  docs: {
    rootPath: path.join(process.cwd(), 'docs'),
    tsConfigPath: 'tsconfig.json',
  },
};
