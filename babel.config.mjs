// @ts-check
import { fileURLToPath } from 'node:url';
import * as path from 'node:path';
import { existsSync } from 'node:fs';
// @ts-ignore
import getBaseConfig from '@mui/internal-code-infra/babel-config';

/**
 * @typedef {import('@babel/core')} babel
 */

const filename = fileURLToPath(import.meta.url);
const dirname = path.dirname(filename);

const errorCodesPath = path.resolve(dirname, './docs/public/static/error-codes.json');

/**
 * @param {string} relativeToBabelConf
 * @returns {string}
 */
function resolveAliasPath(relativeToBabelConf) {
  const resolvedPath = path.relative(process.cwd(), path.resolve(dirname, relativeToBabelConf));
  return `./${resolvedPath.replace('\\', '/')}`;
}

/**
 * @param {string} alias
 * @param {{ subPath?: string }} [options]
 * @returns {string}
 */
function resolveArchivedMuiAlias(alias, options = {}) {
  const { subPath = 'src' } = options;
  const aliasSuffix = alias.replace(/^@mui\//, '');
  const archiveFolder = `./archives/mui-packages/mui-${aliasSuffix}`;
  const candidatePath = subPath ? path.join(archiveFolder, subPath) : archiveFolder;

  if (!existsSync(path.resolve(dirname, candidatePath))) {
    throw new Error(
      `Unable to resolve archived alias for ${alias}. Expected ${candidatePath} to exist.`,
    );
  }

  return resolveAliasPath(candidatePath);
}

/** @type {babel.ConfigFunction} */
export default function getBabelConfig(api) {
  const baseConfig = getBaseConfig(api);
  const useESModules = api.env(['regressions', 'stable']);

  const defaultAlias = {
    // Route Babel's module resolver through the archived packages so Jest and tooling reuse the
    // frozen JavaScript sources instead of the Rust-first crates. The Rust implementations live in
    // `crates/rustic-ui-*` and ship typed shims via `cargo xtask build-docs` before publishing.
    '@mui/material': resolveArchivedMuiAlias('@mui/material'),
    '@mui/docs': resolveArchivedMuiAlias('@mui/docs'),
    '@mui/icons-material': resolveArchivedMuiAlias('@mui/icons-material', {
      subPath: useESModules ? 'lib/esm' : 'lib',
    }),
    '@mui/lab': resolveArchivedMuiAlias('@mui/lab'),
    '@mui/internal-markdown': resolveAliasPath('./packages/markdown'),
    '@mui/styled-engine': resolveArchivedMuiAlias('@mui/styled-engine'),
    '@mui/styled-engine-sc': resolveArchivedMuiAlias('@mui/styled-engine-sc'),
    '@mui/system': resolveArchivedMuiAlias('@mui/system'),
    '@mui/private-theming': resolveArchivedMuiAlias('@mui/private-theming'),
    '@mui/utils': resolveArchivedMuiAlias('@mui/utils'),
    '@mui/joy': resolveArchivedMuiAlias('@mui/joy'),
    '@mui/internal-docs-utils': resolveAliasPath('./packages-internal/docs-utils/src'),
    '@mui/internal-test-utils': resolveAliasPath('./packages-internal/test-utils/src'),
    docs: resolveAliasPath('./docs'),
    test: resolveAliasPath('./test'),
  };

  /** @type {babel.PluginItem[]} */
  const plugins = [
    [
      '@mui/internal-babel-plugin-minify-errors',
      {
        missingError: 'annotate',
        errorCodesPath,
        runtimeModule: '@mui/utils/formatMuiErrorMessage',
      },
    ],
  ];

  if (process.env.NODE_ENV === 'test') {
    plugins.push([
      'babel-plugin-module-resolver',
      {
        alias: defaultAlias,
        root: ['./'],
      },
    ]);
  }
  const basePlugins = (baseConfig.plugins || []).filter(
    (/** @type {[unknown, unknown, string]} */ [, , pluginName]) =>
      pluginName !== '@mui/internal-babel-plugin-display-name',
  );
  basePlugins.push(...plugins);

  return {
    ...baseConfig,
    plugins: basePlugins,
    overrides: [
      {
        exclude: /\.test\.(m?js|ts|tsx)$/,
        plugins: ['@babel/plugin-transform-react-constant-elements'],
      },
    ],
    env: {
      coverage: {
        plugins: [
          'babel-plugin-istanbul',
          [
            'babel-plugin-module-resolver',
            {
              root: ['./'],
              alias: defaultAlias,
            },
          ],
        ],
      },
      development: {
        plugins: [
          [
            'babel-plugin-module-resolver',
            {
              alias: {
                ...defaultAlias,
                modules: './modules',
              },
              root: ['./'],
            },
          ],
        ],
      },
      test: {
        sourceMaps: 'both',
        plugins: [
          [
            'babel-plugin-module-resolver',
            {
              root: ['./'],
              alias: defaultAlias,
            },
          ],
        ],
      },
    },
  };
}
