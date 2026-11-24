#!/bin/sh
set -eu

# Git exports repository-local environment variables while invoking hooks.
# Clear Git's complete local set so nested commands cannot operate on the
# repository whose hook is under test even when `git -C` points elsewhere.
for git_var in $(git rev-parse --local-env-vars); do
	unset "$git_var"
done

test_root=$(mktemp -d "${TMPDIR:-/tmp}/ekodb-hooks-test.XXXXXX")
trap 'rm -rf "$test_root"' EXIT HUP INT TERM

main_repo="$test_root/main"
linked_worktree="$test_root/worktree"

git init --quiet "$main_repo"
git -C "$main_repo" config user.email "hooks-test@example.invalid"
git -C "$main_repo" config user.name "Hook Test"
# Keep the fixture independent from any repository-level or global hooks path
# inherited by the process that launched this test.
mkdir -p "$main_repo/.git/hooks"
git -C "$main_repo" config core.hooksPath "$main_repo/.git/hooks"
mkdir -p "$main_repo/scripts"
cp Makefile "$main_repo/Makefile"
cp scripts/pre-commit "$main_repo/scripts/pre-commit"
chmod +x "$main_repo/scripts/pre-commit"
git -C "$main_repo" add Makefile scripts/pre-commit
git -C "$main_repo" commit --quiet -m "test fixture"
git -C "$main_repo" worktree add --quiet -b hook-test "$linked_worktree"

make --silent -C "$linked_worktree" ensure-hooks
hook_path=$(git -C "$linked_worktree" rev-parse --git-path hooks/pre-commit)
test -L "$hook_path"
test -e "$hook_path"

rm "$hook_path"
ln -s "$test_root/missing-hook" "$hook_path"
if make --silent -C "$linked_worktree" ensure-hooks >/dev/null 2>&1; then
	echo "ensure-hooks accepted a dangling hook symlink" >&2
	exit 1
fi

# install-hooks replaces whatever is at the hook path and never writes through it: a stray symlink there
# to another file must not have that file's content or mode changed.
stray_hook="$main_repo/.git/hooks/pre-commit"
printf 'unchanged\n' > "$main_repo/scripts/other.sh"
chmod 644 "$main_repo/scripts/other.sh"
git -C "$main_repo" add scripts/other.sh
rm -f "$stray_hook"
ln -s ../../scripts/other.sh "$stray_hook"
make --silent -C "$main_repo" install-hooks
test ! -L "$stray_hook"
test -x "$stray_hook"
cmp -s "$main_repo/scripts/pre-commit" "$stray_hook"
test "$(cat "$main_repo/scripts/other.sh")" = unchanged
test ! -x "$main_repo/scripts/other.sh"

# ensure-hooks links the hook to scripts/pre-commit and changes no mode: where that file is itself a link to
# another file, the other file keeps its mode. The umask is fixed because `chmod +x` adds only the bits it
# allows, so under a stricter one the old recipe would change nothing here either.
chain="$test_root/chain"
git init --quiet "$chain"
mkdir -p "$chain/.git/hooks" "$chain/scripts"
git -C "$chain" config core.hooksPath "$chain/.git/hooks"
cp Makefile "$chain/Makefile"
printf '#!/bin/sh\n' > "$chain/scripts/other.sh"
chmod 744 "$chain/scripts/other.sh"
ln -s other.sh "$chain/scripts/pre-commit"
mode_before=$(ls -ld "$chain/scripts/other.sh" | cut -c1-10)
content_before=$(cat "$chain/scripts/other.sh")
(umask 022 && make --silent -C "$chain" ensure-hooks)
test -L "$chain/.git/hooks/pre-commit"
test "$mode_before" = "$(ls -ld "$chain/scripts/other.sh" | cut -c1-10)"
test "$content_before" = "$(cat "$chain/scripts/other.sh")"

# Reproduce the hook context that originally escaped the fixture. The nested
# run must not change the caller-selected repository, index, HEAD, or config.
if [ "${EKODB_HOOK_ENV_PROBE:-0}" != "1" ]; then
	protected_config="$test_root/protected.config"
	protected_config_before="$test_root/protected.config.before"
	printf '[sentinel]\n\tvalue = unchanged\n' > "$protected_config"
	cp "$protected_config" "$protected_config_before"
	protected_head=$(git -C "$main_repo" rev-parse HEAD)
	protected_tree=$(git -C "$main_repo" write-tree)
	GIT_CONFIG="$protected_config" \
		GIT_DIR="$main_repo/.git" \
		GIT_WORK_TREE="$main_repo" \
		GIT_INDEX_FILE="$main_repo/.git/index" \
		EKODB_HOOK_ENV_PROBE=1 "$0" >/dev/null
	cmp -s "$protected_config_before" "$protected_config"
	test "$protected_head" = "$(git -C "$main_repo" rev-parse HEAD)"
	test "$protected_tree" = "$(git -C "$main_repo" write-tree)"
fi

echo "Hook installer worktree checks passed."
