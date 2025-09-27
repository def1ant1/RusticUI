export interface RustDocFlags {
  legacyArchiveRoot: string;
  rustMetadataRoot: string | null;
  preferRust: boolean;
  rustAuthoritative: boolean;
  shouldSkipArchives: boolean;
}

export declare const rustDocFlags: RustDocFlags;

export declare function resolvePackageSourceRoot(packageSlug: string): string;

export declare function describeSourceFor(packageSlug: string): {
  archivePath: string;
  rustPath: string | null;
  activePath: string;
};
