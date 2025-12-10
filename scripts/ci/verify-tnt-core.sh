#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
tnt_core_path="${TNT_CORE_PATH:-"${repo_root}/../tnt-core"}"

if [[ ! -d "${tnt_core_path}" ]]; then
  echo "::error::tnt-core repository not found at ${tnt_core_path}" >&2
  exit 1
fi

if [[ ! -d "${tnt_core_path}/.git" ]]; then
  echo "::error::${tnt_core_path} exists but is not a git checkout" >&2
  exit 1
fi

pushd "${tnt_core_path}" >/dev/null
head_rev="$(git rev-parse HEAD)"
version_file="bindings/TNT_CORE_VERSION"
if [[ ! -f "${version_file}" ]]; then
  echo "::error::${version_file} missing from tnt-core checkout" >&2
  exit 1
fi
recorded_rev="$(< "${version_file}")"

if [[ "${head_rev}" != "${recorded_rev}" ]]; then
  echo "::error::tnt-core bindings generated from ${recorded_rev} but checkout is at ${head_rev}. Run 'cargo xtask gen-bindings' inside tnt-core to refresh them." >&2
  exit 1
fi
popd >/dev/null

echo "tnt-core bindings verified at ${recorded_rev}"
