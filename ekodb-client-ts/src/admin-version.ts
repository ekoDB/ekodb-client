// New permission resources are detected by probing, including prerelease builds.
export const MIN_SERVER_VERSION_FOR_PERMISSION_EDITS: string | undefined =
  "0.80.0";
// Legacy edits preserve omitted restrictions from this stable release onward.
export const MIN_LEGACY_SERVER_VERSION_FOR_PERMISSION_EDITS:
  | string
  | undefined = "0.79.2";
