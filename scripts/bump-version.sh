#!/usr/bin/env bash
#
# Bump the lockstep crates to a new version and verify no stale reference is
# left behind.
#
#   ./scripts/bump-version.sh 0.12.5
#
# cargo-release cannot drive this workspace: release.toml sets
# shared-version = true, but the example-* members carry their own unrelated
# versions, and cargo-progenitor is not publishable under that name because
# crates.io/crates/cargo-progenitor belongs to oxidecomputer. So the bump is
# scripted here instead.
#
# progenitor-middleware-client is deliberately absent from LOCKSTEP: it
# versions independently and has had no functional change since 0.12.0.
#
# Only single-line dependency tables are rewritten, which is every form this
# workspace uses. A dependency split across lines would need widening here.

set -euo pipefail

LOCKSTEP=(
	progenitor-middleware
	progenitor-middleware-impl
	progenitor-middleware-macro
	cargo-progenitor
)

new="${1:-}"
if [[ ! $new =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$ ]]; then
	echo "usage: $0 <semver>" >&2
	exit 2
fi

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

manifests=(Cargo.toml)
while IFS= read -r m; do manifests+=("$m"); done < <(find . -mindepth 2 -maxdepth 2 -name Cargo.toml -not -path './target/*' | sort)

export NEW="$new"
export NAMES="${LOCKSTEP[*]}"

for crate in "${LOCKSTEP[@]}"; do
	perl -pi -e 'if (!$seen && /^version = "/) { s/"[^"]*"/"$ENV{NEW}"/; $seen = 1 }' "$crate/Cargo.toml"
done

perl -pi -e '
	BEGIN { $names = join "|", map { quotemeta } split " ", $ENV{NAMES} }
	s/^(\s*(?:$names)\s*=\s*\{[^}]*?version\s*=\s*")[^"]*(")/$1$ENV{NEW}$2/;
' "${manifests[@]}"

cargo update --workspace --quiet

fail=0
for crate in "${LOCKSTEP[@]}"; do
	got="$(perl -ne 'if (/^version = "([^"]*)"/) { print $1; exit }' "$crate/Cargo.toml")"
	if [[ $got != "$new" ]]; then
		echo "stale package version: $crate/Cargo.toml is $got, expected $new" >&2
		fail=1
	fi
done

while IFS= read -r hit; do
	echo "stale dependency version: $hit (expected $new)" >&2
	fail=1
done < <(perl -ne '
	BEGIN { $names = join "|", map { quotemeta } split " ", $ENV{NAMES} }
	next unless /^\s*($names)\s*=\s*\{[^}]*?version\s*=\s*"([^"]*)"/;
	print "$ARGV:$.: $1 = $2\n" unless $2 eq $ENV{NEW};
' "${manifests[@]}")

if ((fail)); then
	exit 1
fi

echo "bumped ${LOCKSTEP[*]} to $new"
