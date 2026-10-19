import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { resolve, join } from "node:path";
import { expect, it } from "vitest";

it("installs the packed artifact and supports root, admin and existing deep imports", () => {
  const directory = mkdtempSync(join(tmpdir(), "ekodb-admin-package-"));
  const packageRoot = resolve(__dirname, "..");
  const metadata = JSON.parse(
    readFileSync(join(packageRoot, "package.json"), "utf8"),
  ) as { dependencies: { [name: string]: string } };
  // Install copies of the already-installed runtime dependencies. A fresh cache
  // proves this consumer check needs neither registry metadata nor a server.
  const dependencies = Object.keys(metadata.dependencies).map((name) =>
    resolve(packageRoot, "node_modules", name),
  );
  try {
    const output = execFileSync(
      "npm",
      ["pack", "--json", "--pack-destination", directory],
      { cwd: packageRoot, encoding: "utf8" },
    );
    // npm can print prepare output before its JSON report.
    const report = JSON.parse(output.slice(output.indexOf("[\n"))) as {
      filename: string;
    }[];
    writeFileSync(
      join(directory, "package.json"),
      JSON.stringify({ private: true }),
    );
    execFileSync(
      "npm",
      [
        "install",
        "--offline",
        "--install-links",
        "--cache",
        join(directory, "npm-cache"),
        "--ignore-scripts",
        "--no-audit",
        "--no-fund",
        join(directory, report[0].filename),
        ...dependencies,
      ],
      { cwd: directory, stdio: "pipe" },
    );
    writeFileSync(
      join(directory, "consumer.cjs"),
      `
      const assert = require('node:assert/strict');
      const {EkoDBClient} = require('@ekodb/ekodb-client');
      const {EkoDBAdminClient, MIN_SERVER_VERSION_FOR_PERMISSION_EDITS} = require('@ekodb/ekodb-client/admin');
      assert.equal(MIN_SERVER_VERSION_FOR_PERMISSION_EDITS, undefined);
      assert.equal(typeof EkoDBClient, 'function');
      assert.equal(typeof EkoDBAdminClient, 'function');
      assert.equal(typeof require('@ekodb/ekodb-client/dist/client').EkoDBClient, 'function');
      assert.equal(require('@ekodb/ekodb-client/dist').EkoDBClient, EkoDBClient);
      assert.equal(typeof require('@ekodb/ekodb-client/dist/client.js').EkoDBClient, 'function');
      assert.equal(require('@ekodb/ekodb-client/package.json').name, '@ekodb/ekodb-client');
      assert.equal(new EkoDBAdminClient({baseURL:'https://example.test',apiKey:'test'}).insert, undefined);
      console.log('CommonJS OK');
    `,
    );
    writeFileSync(
      join(directory, "consumer.mjs"),
      `
      import assert from 'node:assert/strict';
      import {EkoDBClient} from '@ekodb/ekodb-client';
      import {EkoDBAdminClient, MIN_SERVER_VERSION_FOR_PERMISSION_EDITS} from '@ekodb/ekodb-client/admin';
      assert.equal(MIN_SERVER_VERSION_FOR_PERMISSION_EDITS, undefined);
      import {EkoDBClient as DeepClient} from '@ekodb/ekodb-client/dist/client.js';
      assert.equal(typeof EkoDBClient, 'function');
      assert.equal(typeof EkoDBAdminClient, 'function');
      assert.equal(DeepClient, EkoDBClient);
      console.log('ESM OK');
    `,
    );
    const typed = `
      import {EkoDBClient} from '@ekodb/ekodb-client';
      import {EkoDBAdminClient, type CreateApiKeyRequest, type ApiKey, type UpdateApiKeyCollectionPermissionsRequest, MIN_SERVER_VERSION_FOR_PERMISSION_EDITS} from '@ekodb/ekodb-client/admin';
      import {EkoDBClient as DeepClient} from '@ekodb/ekodb-client/dist/client';
      const client = new EkoDBAdminClient({baseURL:'https://example.test',apiKey:'test'});
      const request: CreateApiKeyRequest = {collection_permissions:{users:['read']}};
      const result: Promise<ApiKey> = client.createApiKey(request);
      new EkoDBClient({baseURL:'https://example.test',apiKey:'test'});
      // @ts-expect-error An admin facade has no CRUD methods.
      client.insert('users', {});
      const edit: UpdateApiKeyCollectionPermissionsRequest = {api_key:'test', collection:'users', read:true, write:false, select_fields:null, exclude_fields:undefined, writable_fields:[]};
      const minimum: string | undefined = MIN_SERVER_VERSION_FOR_PERMISSION_EDITS;
      void edit;
      void minimum;
      void result;
      void DeepClient;
    `;
    writeFileSync(join(directory, "consumer.cts"), typed);
    writeFileSync(join(directory, "consumer.mts"), typed);
    expect(
      execFileSync(process.execPath, ["consumer.cjs"], {
        cwd: directory,
        encoding: "utf8",
      }),
    ).toContain("CommonJS OK");
    expect(
      execFileSync(process.execPath, ["consumer.mjs"], {
        cwd: directory,
        encoding: "utf8",
      }),
    ).toContain("ESM OK");
    execFileSync(
      resolve(packageRoot, "node_modules/.bin/tsc"),
      [
        "--noEmit",
        "--strict",
        "--exactOptionalPropertyTypes",
        "--target",
        "ES2020",
        "--module",
        "nodenext",
        "--moduleResolution",
        "nodenext",
        "consumer.cts",
        "consumer.mts",
      ],
      { cwd: directory, stdio: "pipe" },
    );
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}, 60_000);
