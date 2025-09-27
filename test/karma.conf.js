const fs = require('fs');
const path = require('path');
const playwright = require('@playwright/test');
const webpack = require('webpack');

const CI = Boolean(process.env.CI);

const workspaceRoot = path.resolve(__dirname, '..');
const archivedMuiRoot = path.join(workspaceRoot, 'archives', 'mui-packages');

function resolveArchivedMuiAlias(alias, subPath = 'src') {
  const aliasSuffix = alias.replace(/^@mui\//, '');
  const archiveFolder = path.join(archivedMuiRoot, `mui-${aliasSuffix}`);
  const candidate = subPath ? path.join(archiveFolder, subPath) : archiveFolder;

  if (!fs.existsSync(candidate)) {
    throw new Error(
      `Unable to resolve archived alias for ${alias}. Expected ${candidate} to exist.\n` +
        'Run `cargo xtask build-docs` to regenerate the Rust-owned shims if the archive moved.',
    );
  }

  const relative = path.relative(workspaceRoot, candidate).replace(/\\/g, '/');
  return `./${relative}`;
}

let build = `material-ui local ${new Date().toISOString()}`;

if (process.env.CIRCLECI) {
  const buildPrefix =
    process.env.CIRCLE_PR_NUMBER !== undefined
      ? process.env.CIRCLE_PR_NUMBER
      : process.env.CIRCLE_BRANCH;
  build = `${buildPrefix}: ${process.env.CIRCLE_BUILD_URL}`;
}

const browserStack = {
  // |commits in PRs| >> |Merged commits|.
  // Since we have limited resources on BrowserStack we often time out on PRs.
  // However, BrowserStack rarely fails with a true-positive so we use it as a stop gap for release not merge.
  // But always enable it locally since people usually have to explicitly have to expose their BrowserStack access key anyway.
  enabled:
    !process.env.CI ||
    process.env.BROWSERSTACK_FORCE === 'true' ||
    (process.env.BROWSERSTACK_ENABLED === 'true' &&
      process.env.CIRCLE_BRANCH.match(/^(master|next|v\d+\.x)$/)),
  username: process.env.BROWSERSTACK_USERNAME,
  accessKey: process.env.BROWSERSTACK_ACCESS_KEY,
  build,
  // https://github.com/browserstack/api#timeout300
  timeout: 5 * 60, // Maximum time before a worker is terminated. Default 5 minutes.
};

process.env.CHROME_BIN = playwright.chromium.executablePath();
process.env.FIREFOX_BIN = playwright.firefox.executablePath();

// BrowserStack rate limit after 1600 calls every 5 minutes.
// Per second, https://www.browserstack.com/docs/automate/api-reference/selenium/introduction#rest-api-projects
const MAX_REQUEST_PER_SECOND_BROWSERSTACK = 1600 / (60 * 5);
// Estimate the max number of concurrent karma builds
// For each PR, 6 concurrent builds are used, only one is using BrowserStack.
const AVERAGE_KARMA_BUILD = 1 / 6;
// CircleCI accepts up to 83 concurrent builds.
const MAX_CIRCLE_CI_CONCURRENCY = 83;

// Karma configuration
module.exports = function setKarmaConfig(config) {
  const baseConfig = {
    basePath: '../',
    browsers: ['chromeHeadless', 'FirefoxHeadless'],
    browserDisconnectTimeout: 3 * 60 * 1000, // default 2000
    browserDisconnectTolerance: 1, // default 0
    browserNoActivityTimeout: 3 * 60 * 1000, // default 30000
    colors: true,
    coverageIstanbulReporter: {
      combineBrowserReports: true,
      dir: path.resolve(__dirname, '../coverage'),
      fixWebpackSourcePaths: true,
      reports: CI ? ['lcov'] : [],
      skipFilesWithNoCoverage: true,
      verbose: false,
    },
    client: {
      mocha: {
        // Some BrowserStack browsers can be slow.
        timeout: (process.env.CIRCLECI === 'true' ? 6 : 2) * 1000,
      },
    },
    frameworks: ['mocha', 'webpack'],
    files: [
      {
        pattern: 'test/karma.tests.js',
        watched: false,
      },
      {
        pattern: 'test/assets/*.png',
        watched: false,
        included: false,
        served: true,
      },
    ],
    plugins: [
      'karma-mocha',
      'karma-chrome-launcher',
      'karma-coverage-istanbul-reporter',
      'karma-sourcemap-loader',
      'karma-webpack',
      'karma-firefox-launcher',
      'karma-spec-reporter',
    ],
    /**
     * possible values:
     * - config.LOG_DISABLE
     * - config.LOG_ERROR
     * - config.LOG_WARN
     * - config.LOG_INFO
     * - config.LOG_DEBUG
     */
    logLevel: config.LOG_INFO,
    port: 9876,
    preprocessors: {
      'test/karma.tests.js': ['webpack', 'sourcemap'],
    },
    proxies: {
      '/fake.png': '/base/test/assets/fake.png',
      '/fake2.png': '/base/test/assets/fake2.png',
    },
    // The CI branch fixes double log issue
    // https://github.com/karma-runner/karma/issues/2342
    reporters: CI ? ['dots', 'coverage-istanbul'] : ['spec'],
    webpack: {
      mode: 'development',
      devtool: CI ? 'inline-source-map' : 'eval-source-map',
      optimization: {
        nodeEnv: 'test',
      },
      plugins: [
        new webpack.DefinePlugin({
          'process.env.CI': JSON.stringify(process.env.CI),
          'process.env.KARMA': JSON.stringify(true),
          'process.env.TEST_GATE': JSON.stringify(process.env.TEST_GATE),
        }),
        new webpack.ProvidePlugin({
          // required by code accessing `process.env` in the browser
          process: 'process/browser.js',
        }),
      ],
      module: {
        rules: [
          {
            test: /\.(js|mjs|ts|tsx)$/,
            loader: 'babel-loader',
            // assertion-error uses static initialization blocks, which doesn't work in Safari 15 on BrowserStack
            exclude: /node_modules\/(.*\/)?(?!assertion-error)\//,
            options: {
              envName: 'stable',
            },
          },
          // transpile 3rd party packages with dependencies in this repository
          {
            test: /\.(js|mjs|jsx)$/,
            include:
              /node_modules(\/|\\)(notistack|@mui(\/|\\)x-data-grid|@mui(\/|\\)x-data-grid-pro|@mui(\/|\\)x-license-pro|@mui(\/|\\)x-data-grid-generator|@mui(\/|\\)x-date-pickers-pro|@mui(\/|\\)x-date-pickers|@mui(\/|\\)x-tree-view)/,
            use: {
              loader: 'babel-loader',
              options: {
                // We have to apply `babel-plugin-module-resolve` to the files in `@mui/x-date-pickers`.
                // Otherwise we can't import `@mui/material` from `@mui/x-date-pickers` in `pnpm test:karma`.
                sourceType: 'unambiguous',
                plugins: [
                  [
                    'babel-plugin-module-resolver',
                    {
                      alias: {
                        // all packages in this monorepo. The canonical implementations are Rust crates
                        // (`crates/rustic-ui-*`) that surface typed shims via `cargo xtask build-docs`. Karma
                        // continues to transpile the archived JavaScript snapshots for regression parity.
                        '@mui/material': resolveArchivedMuiAlias('@mui/material'),
                        '@mui/docs': resolveArchivedMuiAlias('@mui/docs'),
                        '@mui/icons-material': resolveArchivedMuiAlias('@mui/icons-material', 'lib'),
                        '@mui/lab': resolveArchivedMuiAlias('@mui/lab'),
                        '@mui/styled-engine': resolveArchivedMuiAlias('@mui/styled-engine'),
                        '@mui/system': resolveArchivedMuiAlias('@mui/system'),
                        '@mui/private-theming': resolveArchivedMuiAlias('@mui/private-theming'),
                        '@mui/utils': resolveArchivedMuiAlias('@mui/utils'),
                        '@mui/material-nextjs': resolveArchivedMuiAlias('@mui/material-nextjs'),
                        '@mui/joy': resolveArchivedMuiAlias('@mui/joy'),
                      },
                      transformFunctions: ['require'],
                    },
                  ],
                ],
              },
            },
          },
          {
            test: /\.(js|mjs|ts|tsx)$/,
            use: {
              loader: 'babel-loader',
              options: {
                plugins: ['babel-plugin-istanbul'],
              },
            },
            enforce: 'post',
            exclude: /node_modules/,
          },
        ],
      },
      resolve: {
        extensions: ['.js', '.mjs', '.ts', '.tsx'],
        fallback: {
          // needed by sourcemap
          fs: false,
          path: false,
          // Exclude polyfill and treat 'stream' as an empty module since it is not required. next -> gzip-size relies on it.
          stream: false,
          vm: false,
        },
      },
      // TODO: 'browserslist:modern'
      // See https://github.com/webpack/webpack/issues/14203
      target: 'web',
    },
    customLaunchers: {
      chromeHeadless: {
        base: 'ChromeHeadless',
        flags: ['--no-sandbox', '--use-mock-keychain'],
      },
    },
    singleRun: CI,
  };

  let newConfig = baseConfig;

  if (browserStack.enabled && browserStack.accessKey) {
    newConfig = {
      ...baseConfig,
      browserStack,
      browsers: baseConfig.browsers.concat(['chrome', 'safari', 'edge']),
      plugins: baseConfig.plugins.concat(['karma-browserstack-launcher']),
      customLaunchers: {
        ...baseConfig.customLaunchers,
        chrome: {
          base: 'BrowserStack',
          os: 'OS X',
          os_version: 'Monterey',
          browser: 'chrome',
          // We support Chrome 109.x per .browserslistrc
          browser_version: '109.0',
        },
        safari: {
          base: 'BrowserStack',
          os: 'OS X',
          os_version: 'Monterey',
          browser: 'safari',
          browser_version: '15.6',
        },
        edge: {
          base: 'BrowserStack',
          os: 'Windows',
          os_version: '10',
          browser: 'edge',
          browser_version: '120.0',
        },
      },
    };

    // -1 because chrome headless runs in the local machine
    const browserStackBrowsersUsed = newConfig.browsers.length - 1;

    // default 1000, Avoid Rate Limit Exceeded
    newConfig.browserStack.pollingTimeout =
      ((MAX_CIRCLE_CI_CONCURRENCY * AVERAGE_KARMA_BUILD * browserStackBrowsersUsed) /
        MAX_REQUEST_PER_SECOND_BROWSERSTACK) *
      1000;
  }

  config.set(newConfig);
};
