/**
 * Minimal static file server for Storybook exports.
 *
 * Storybook's static output is self-contained, so rather than depending on an
 * additional dev-server dependency we spin up a small HTTP server per Storybook.
 * The server intentionally only implements features required by Storybook:
 *
 * - Serves `index.html`, `iframe.html`, and hashed asset files.
 * - Sets `Content-Type` headers for the most common extensions so the browser
 *   applies the right parser and caches consistently across runs.
 * - Logs incoming requests when `DEBUG_ADAPTER_SERVER=1` to ease debugging in CI.
 */

import * as fs from 'node:fs/promises';
import * as http from 'node:http';
import * as path from 'node:path';
import { AddressInfo } from 'node:net';

const MIME_TYPES: Record<string, string> = {
  '.css': 'text/css; charset=utf-8',
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
  '.svg': 'image/svg+xml',
  '.woff': 'font/woff',
  '.woff2': 'font/woff2',
};

function resolveContentType(filePath: string): string | undefined {
  return MIME_TYPES[path.extname(filePath)] ?? MIME_TYPES['.html'];
}

async function readFileWithFallback(root: string, requestPath: string): Promise<{ data: Buffer; filePath: string }> {
  const normalizedPath = requestPath === '/' ? '/index.html' : requestPath;
  const candidatePaths = [normalizedPath, `${normalizedPath}/index.html`];

  for (const candidate of candidatePaths) {
    const absolutePath = path.join(root, candidate);
    try {
      const data = await fs.readFile(absolutePath);
      return { data, filePath: absolutePath };
    } catch (error: unknown) {
      if ((error as NodeJS.ErrnoException).code === 'ENOENT') {
        continue;
      }
      throw error;
    }
  }

  throw Object.assign(new Error(`File not found: ${requestPath}`), { statusCode: 404 });
}

export interface StaticServer {
  readonly baseUrl: string;
  stop(): Promise<void>;
}

/**
 * Start the HTTP server and return a handle that exposes the base URL.
 */
export async function startStaticServer(root: string): Promise<StaticServer> {
  const server = http.createServer(async (request, response) => {
    const method = request.method ?? 'GET';
    const url = request.url ?? '/';

    if (process.env.DEBUG_ADAPTER_SERVER === '1') {
      console.log(`[adapter-static] ${method} ${url}`);
    }

    if (method !== 'GET' && method !== 'HEAD') {
      response.statusCode = 405;
      response.end('Method Not Allowed');
      return;
    }

    try {
      const { data, filePath } = await readFileWithFallback(root, new URL(url, 'http://localhost').pathname);
      const contentType = resolveContentType(filePath);
      if (contentType) {
        response.setHeader('Content-Type', contentType);
      }
      response.statusCode = 200;
      response.end(method === 'HEAD' ? undefined : data);
    } catch (error: unknown) {
      const statusCode = (error as { statusCode?: number }).statusCode ?? 500;
      response.statusCode = statusCode;
      response.end(statusCode === 404 ? 'Not Found' : 'Internal Server Error');
      if (statusCode >= 500) {
        console.error('[adapter-static] failed to serve request', error);
      }
    }
  });

  await new Promise<void>((resolve) => {
    server.listen(0, '127.0.0.1', resolve);
  });

  const { port } = server.address() as AddressInfo;
  const baseUrl = `http://127.0.0.1:${port}`;

  return {
    baseUrl,
    async stop() {
      await new Promise<void>((resolve, reject) => {
        server.close((error) => {
          if (error) {
            reject(error);
          } else {
            resolve();
          }
        });
      });
    },
  };
}
