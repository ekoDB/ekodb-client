#!/usr/bin/env bash
# A clean checkout stays clean after a make target (#411). Git restores each tracked file's mode, so a recipe
# that chmods a tracked file leaves a mode change in the tree, and a later `git add -A` commits it into unrelated
# work. This check keeps `chmod` in the Makefile from changing the mode git tracks for any file.
#
# It is a whitelist, so it has no list of forms to refuse. Every Makefile line that holds the word `chmod` (a
# backslash continuation joined as make and the shell join it, so a word split across lines is one word; a
# line that is only a comment skipped, while a comment after other text on a line that holds `chmod` is part of
# the line) must be one of two things:
#
#   1. A plain recipe command: a tab, optional `@`, `+` or `-`, `chmod`, one mode, then one or more literal
#      paths, separated by single spaces, and nothing else (trailing spaces and tabs aside). The mode is three or four octal digits or one
#      symbolic clause (`+x`, `u=rw`, `g-w`). Each path must name a tracked regular file exactly: spelled as
#      git records it, reached without passing through a symlink, present on disk, and with no other hard link.
#      The file must keep the mode git records for it: the owner execute bit is set afterwards exactly when it
#      is tracked 100755, or is left alone. Anything else is not plain: an untracked path, a directory, a path
#      that does not exist yet (an earlier line may create it as a link), a symlink or a path through one, a
#      hard-linked file, a path inside `.git`, a glob, an operand that starts with `-` (chmod reads that as an
#      option wherever it stands), a variable, a quote, a redirection, a second command, an absolute path, or a
#      doubled space.
#   2. A line approved in scripts/test-script-modes.allow, written there as it stands in the Makefile with its
#      continuations joined as above (leading and trailing whitespace aside; the check prints a line when it
#      refuses it), for the cases this check cannot read or cannot settle. A person reads the line and approves
#      what it runs, and answers for any state it depends on that the check cannot read. A row that matches no
#      line fails, so an approval does not outlive its line.
#
# Any other line fails, with the line and what to do. The tracked set is read from the checkout's own index,
# with GIT_DIR, GIT_INDEX_FILE and GIT_WORK_TREE removed from the environment of the git call. A checkout
# with a `GNUmakefile` or a `makefile` (in any letter case) fails outright: make reads either ahead of
# `Makefile`, so this check of `Makefile` would say nothing about what runs. So does a Makefile that holds a
# carriage return, which make reads as an ordinary byte and a text reader as a line break. So does a Makefile that sets
# `.ONESHELL` or `.RECIPEPREFIX` (as a target, in a target list, or as a variable, or under a name that
# holds a `$`, which make can expand into one), which change how recipe lines run: under `.ONESHELL` a `cd` on one line
# carries to the chmod on the next, and the path the check resolved is not the path chmod gets.
#
# What it settles is each chmod line against the tree as it is when the check runs, and against the mode as
# the line is written. Not covered:
#   - a command whose name is never spelled `chmod` on one line (`$(CHMOD)`, `c"h"mod`, a name assembled from
#     variables), `install -m`, a script a recipe runs that chmods on its own, and a file the Makefile
#     includes: only the Makefile itself is read;
#   - what program the recipe's `chmod` resolves to (an exported PATH or SHELL, BASH_ENV, a tracked wrapper named
#     chmod): the check trusts the Makefile's author not to redirect the command. It guards the accident that
#     started it (a glob or a mode that changes a tracked file's mode), not a Makefile written to defeat it, and
#     a redirect that changes no mode (a different chmod binary with the same effect) is not a loss;
#   - another recipe line that changes the tree before the chmod runs (a link, copy or move made over a path
#     the chmod names), because no line but the chmod is read;
#   - a symbolic mode with no who (`+x`, `=x`), which is read as `a`: that holds unless the umask clears the
#     owner execute bit, which no environment this runs in does.
#
#   bash scripts/test-script-modes.sh              check this checkout's Makefile
#   bash scripts/test-script-modes.sh --self-test  run the check against built fixtures
set -euo pipefail
top=$(env -u GIT_DIR -u GIT_INDEX_FILE -u GIT_WORK_TREE git rev-parse --show-toplevel 2>/dev/null) || {
  echo "test-script-modes: not inside a git checkout" >&2
  exit 2
}
cd "$top"
[ -f Makefile ] || {
  echo "test-script-modes: no Makefile at $top" >&2
  exit 2
}
exec python3 - "$@" <<'PY'
import os
import re
import shutil
import subprocess
import sys
import tempfile

# The git variables a hook or a wrapper may export; the tracked set comes from the checkout's own index.
AMBIENT_GIT = ("GIT_DIR", "GIT_INDEX_FILE", "GIT_WORK_TREE")
ALLOW = "scripts/test-script-modes.allow"
NAMES_CHMOD = re.compile(r"\bchmod\b")
MODE = r"(?:[0-7]{3,4}|[ugoa]*[-+=][rwx]+)"
SEGMENT = r"[A-Za-z0-9_.+@,%=:-]+"
PATH = rf"(?!-){SEGMENT}(?:/{SEGMENT})*"
PLAIN = re.compile(rf"^\t[@+-]*chmod ({MODE}) ({PATH}(?: {PATH})*)[ \t]*$")
RECIPE_DIRECTIVES = (".ONESHELL", ".RECIPEPREFIX")
SYMBOLIC = re.compile(r"([ugoa]*)([-+=])([rwx]+)")


def clean_git_env():
    return {k: v for k, v in os.environ.items() if k not in AMBIENT_GIT}


def tracked_modes(root):
    out = subprocess.run(
        ["git", "-C", root, "ls-files", "-s", "-z"], capture_output=True, check=True, env=clean_git_env()
    ).stdout
    modes = {}
    for entry in out.split(b"\0"):
        if entry:
            meta, path = entry.split(b"\t", 1)
            modes[os.fsdecode(path)] = meta.split()[0].decode()
    return modes


def logical_lines(lines):
    """(number, text) of each line with its backslash continuations joined as make and the shell join them. A
    comment on one line is dropped; a comment that continues onto the next line is kept, because in a recipe the
    shell runs that next line."""
    out, i = [], 0
    while i < len(lines):
        number, line, joined = i + 1, lines[i], False
        recipe = line.startswith("\t")
        while line.endswith("\\") and i + 1 < len(lines):
            i += 1
            if recipe:
                # make drops one leading tab of a continuation line, and the shell drops the backslash-newline,
                # so a word split across lines is one word
                line = line[:-1] + (lines[i][1:] if lines[i].startswith("\t") else lines[i])
            else:
                line = line[:-1].rstrip() + " " + lines[i].strip()
            joined = True
        if joined or not line.lstrip().startswith("#"):
            out.append((number, line))
        i += 1
    return out


def sets_recipe_directive(text):
    """Whether a logical line (not a recipe line, not a comment) names .ONESHELL or .RECIPEPREFIX as a target, in a
    target list, as a variable, or as the first word of a target-specific assignment."""
    if text.startswith("\t") or text.lstrip().startswith("#"):
        return False
    name_part = re.split(r"[=:]", text, maxsplit=1)[0]
    if any(word in RECIPE_DIRECTIVES for word in name_part.split()):
        return True
    if "$" in name_part:
        # make forms a special-target or variable name by expansion only through `$`
        return True
    rest = text.split(":", 1)[1] if ":" in text else ""
    words = [word for word in rest.split() if word not in ("override", "export", "private")]
    return bool(words) and (words[0] in RECIPE_DIRECTIVES or ("$" in words[0] and "=" in rest))


MAKE_EVAL = re.compile(r"[$][({]\s*eval\b")


def builds_make_text(text):
    """Whether a logical line (any kind but a whole-line comment) calls make's eval, the one way make text, and so a
    special target, can be built when make runs."""
    if not text.startswith("\t") and text.lstrip().startswith("#"):
        # a make comment; a tab-led line is recipe text, which make expands even when the shell reads it as a comment
        return False
    return bool(MAKE_EVAL.search(text))


def user_exec(mode):
    """1 or 0 when the mode sets or clears the owner execute bit, None when it leaves the bit alone."""
    if mode.isdigit():
        return (int(mode, 8) >> 6) & 1
    who, op, perms = SYMBOLIC.fullmatch(mode).groups()
    if who and "u" not in who and "a" not in who:
        return None
    has_x = "x" in perms
    if op == "=":
        return int(has_x)
    return int(op == "+") if has_x else None


def read_allow(root):
    path = os.path.join(root, ALLOW)
    if not os.path.exists(path):
        return []
    with open(path) as f:
        text = f.read()
    return [
        (number, line.strip())
        for number, line in enumerate(text.split("\n"), 1)
        if line.strip() and not line.strip().startswith("#")
    ]


def operand_problem(root, path, owner_exec, tracked):
    """Why this chmod operand is not settled, or None. It must name a tracked regular file exactly, reached
    without a symlink and with no other hard link, and chmod must leave the file's mode as git records it."""
    resolved = os.path.relpath(os.path.realpath(os.path.join(root, path)), os.path.realpath(root))
    if resolved != path:
        return f"{path} is not spelled as git records it (it resolves to {resolved}: a symlink, `.` or `..`); approve the line"
    mode = tracked.get(resolved)
    if mode not in ("100644", "100755"):
        return f"{path} is not a tracked file; approve the line"
    try:
        links = os.lstat(os.path.join(root, resolved)).st_nlink
    except OSError:
        return f"{path} is tracked but not on disk; approve the line"
    if links > 1:
        return f"{path} has {links} hard links, so chmod changes the other names too; approve the line"
    if owner_exec is not None and bool(owner_exec) != (mode == "100755"):
        after = "executable" if owner_exec else "not executable"
        return f"{path} is tracked {mode}, and this chmod would leave it {after}"
    return None


def check(root):
    shadowing = sorted(n for n in os.listdir(root) if n.lower() in ("gnumakefile", "makefile") and n != "Makefile")
    if shadowing:
        return [f"{name}: make reads it ahead of Makefile, so this check cannot settle what runs; remove it" for name in shadowing], []
    with open(os.path.join(root, "Makefile"), newline="") as f:
        text = f.read()
    if "\r" in text:
        number = text[: text.index("\r")].count("\n") + 1
        return [f"Makefile:{number}: a carriage return: make reads it as an ordinary byte and this check as a line break; remove it"], []
    lines = text.split("\n")
    tracked = tracked_modes(root)
    allow = read_allow(root)
    approved = {text for _, text in allow}
    used, failures, notes = set(), [], []
    failures.extend(
        f"Makefile:{number}: {line.strip()}: this directive changes how recipe lines run, so no chmod line can be settled; remove it"
        for number, line in logical_lines(lines)
        if sets_recipe_directive(line)
    )
    # logical_lines drops a comment line, but make expands a tab-led one (builds_make_text skips the others), so
    # every comment line is read as well
    recipe_comments = [(n, l) for n, l in enumerate(lines, 1) if l.lstrip().startswith("#")]
    failures.extend(
        f"Makefile:{number}: {line.strip()}: eval builds make text when make runs, so no chmod line can be settled; remove it"
        for number, line in [*logical_lines(lines), *recipe_comments]
        if builds_make_text(line)
    )
    for number, text in logical_lines(lines):
        if not NAMES_CHMOD.search(text):
            continue
        where = f"Makefile:{number}"
        if text.strip() in approved:
            used.add(text.strip())
            notes.append(f"{where}: approved in {ALLOW}")
            continue
        plain = PLAIN.match(text)
        if plain:
            owner_exec = user_exec(plain.group(1))
            for path in plain.group(2).split(" "):
                problem = operand_problem(root, path, owner_exec, tracked)
                if problem:
                    failures.append(f"{where}: {problem}: {text.strip()}")
            continue
        failures.append(
            f"{where}: cannot read this line that holds chmod; write it as a plain `chmod <mode> <path>`, or "
            f"approve it in {ALLOW}: {text.strip()}"
        )
    for number, text in allow:
        if text not in used:
            failures.append(f"{ALLOW}:{number}: this row matches no Makefile line that holds chmod: {text}")
    return failures, notes


def self_test():
    def recipe(command):
        return f"t:\n\t{command}\n"

    approved_line = '@set -eu; P=x; chmod +x "$$P"; echo ok'
    cases = [
        # (Makefile text, allow file text or None, whether the check must fail)
        (recipe("chmod +x scripts/x.sh"), None, False),
        (recipe("@chmod +x scripts/x.sh"), None, False),
        (recipe("-chmod +x scripts/x.sh"), None, False),
        (recipe("+chmod +x scripts/x.sh"), None, False),
        (recipe("chmod 755 scripts/x.sh"), None, False),
        (recipe("chmod 0755 scripts/x.sh"), None, False),
        (recipe("chmod 644 scripts/a.sh"), None, False),
        (recipe("chmod -x scripts/a.sh"), None, False),
        (recipe("chmod g+x scripts/a.sh"), None, False),
        (recipe("chmod o+x scripts/a.sh"), None, False),
        (recipe("chmod u=rw scripts/a.sh"), None, False),
        (recipe("chmod g+x scripts/x.sh"), None, False),
        (recipe("chmod +r scripts/a.sh"), None, False),
        (recipe("chmod -r scripts/x.sh"), None, False),
        (recipe("chmod 001 scripts/a.sh"), None, False),
        (recipe("chmod +x scripts/x.sh tools/run.sh"), None, False),
        (recipe("chmod +x scripts/x.sh tools/run.sh scripts/y.sh"), None, False),
        (recipe("chmod +x scripts/run_it.sh"), None, False),
        (recipe("chmod +x scripts/v2.sh"), None, False),
        (recipe("chmod +x scripts/NEW.sh"), None, False),
        (recipe("chmod +x scripts/new-file.sh"), None, False),
        (recipe("chmod +x scripts/c++.sh"), None, False),
        (recipe("chmod +x scripts/a@b.sh"), None, False),
        (recipe("chmod +x scripts/a,b.sh"), None, False),
        (recipe("chmod +x scripts/a%b.sh"), None, False),
        (recipe("chmod +x scripts/a=b.sh"), None, False),
        (recipe("chmod +x scripts/a:b.sh"), None, False),
        (recipe("chmod +x scripts/x.sh \\\n\ttools/run.sh"), None, False),
        ("# chmod +x scripts/a.sh\nt:\n\t@echo ok\n", None, False),
        (recipe("# chmod +x scripts/a.sh"), None, False),
        (recipe("@echo ok"), None, False),
        (recipe(approved_line), f"# reviewed\n{approved_line}\n", False),
        ('t:\n\t@set -eu; \\\n\tchmod +x "$$P"\n', '@set -eu; chmod +x "$$P"\n', False),
        (recipe("chmod +x .git/hooks/pre-commit"), "chmod +x .git/hooks/pre-commit\n", False),
        # a mode that changes what git records
        (recipe("chmod +x scripts/a.sh"), None, True),
        (recipe("chmod 755 scripts/a.sh"), None, True),
        (recipe("chmod =x scripts/a.sh"), None, True),
        (recipe("chmod u+x scripts/a.sh"), None, True),
        (recipe("chmod a+x scripts/a.sh"), None, True),
        (recipe("chmod 705 scripts/a.sh"), None, True),
        (recipe("chmod -x scripts/x.sh"), None, True),
        (recipe("chmod 644 scripts/x.sh"), None, True),
        (recipe("chmod u-x scripts/x.sh"), None, True),
        (recipe("chmod =r scripts/x.sh"), None, True),
        (recipe("chmod 070 scripts/x.sh"), None, True),
        (recipe("chmod +x scripts/x.sh scripts/a.sh"), None, True),
        (recipe("chmod +x Docs/Run.sh"), None, True),
        # not a tracked regular file, spelled as git records it
        (recipe("chmod +x Scripts/A.SH"), None, True),
        (recipe("chmod +x docs/run.sh"), None, True),
        (recipe("chmod +x scripts/untracked.sh"), None, True),
        (recipe("chmod -x scripts/untracked.sh"), None, True),
        (recipe("chmod +x tools"), None, True),
        (recipe("chmod +x scripts/ghost.sh"), None, True),
        (recipe("chmod +x ~/scripts/x.sh"), None, True),
        (recipe("chmod +x scripts/a~b.sh"), None, True),
        (recipe("chmod +x Missing/Ab.sh"), None, True),
        (recipe("chmod +x .git/hooks/pre-commit"), None, True),
        (recipe("chmod +x .GIT/config"), None, True),
        (recipe("chmod +x dotgit/config"), None, True),
        # a symlink, or a path through one, tracked or not, present or not
        (recipe("chmod +x scripts/link"), None, True),
        (recipe("chmod -x scripts/link"), None, True),
        (recipe("chmod +x scripts/link-a"), None, True),
        (recipe("chmod +x scripts/gone"), None, True),
        (recipe("chmod -x scripts/gone"), None, True),
        (recipe("chmod +x scripts/dirlink/run.sh"), None, True),
        (recipe("chmod -x scripts/dirlink/run.sh"), None, True),
        (recipe("chmod +x scripts/ulink"), None, True),
        (recipe("chmod -x scripts/ulink"), None, True),
        (recipe("chmod +x scripts/ulink-a"), None, True),
        (recipe("chmod +x scripts/udir/run.sh"), None, True),
        (recipe("chmod -x scripts/udir/run.sh"), None, True),
        # tracked as a regular file, but a symlink (or a directory that is one) on disk
        (recipe("chmod +x swap/run.sh"), None, True),
        (recipe("chmod +x scripts/swapfile.sh"), None, True),
        # tracked as a symlink or as a submodule, a regular file on disk
        (recipe("chmod -x scripts/symreg.sh"), None, True),
        (recipe("chmod +x scripts/symreg.sh"), None, True),
        (recipe("chmod -x scripts/subm.sh"), None, True),
        (recipe("chmod +x scripts/subm.sh"), None, True),
        # a file with another hard link, tracked or not
        (recipe("chmod +x scripts/h1.sh"), None, True),
        (recipe("chmod +x scripts/h2.sh"), None, True),
        (recipe("chmod +x scripts/h3.sh"), None, True),
        (recipe("chmod +x scripts/h4.sh"), None, True),
        (recipe("chmod +x scripts/h5.sh"), None, True),
        # an operand that starts with `-`, `..`, and the shapes that are not plain
        (recipe("chmod +x -R scripts/x.sh"), None, True),
        (recipe("chmod +x --recursive scripts/x.sh"), None, True),
        (recipe("chmod +x -- scripts/x.sh"), None, True),
        (recipe("chmod +x scripts/x.sh -R"), None, True),
        (recipe("chmod -R +x scripts"), None, True),
        (recipe("chmod +x ../a.sh"), None, True),
        (recipe("chmod +x scripts/../scripts/a.sh"), None, True),
        (recipe("chmod -x scripts/../scripts/a.sh"), None, True),
        (recipe("chmod +x ./scripts/x.sh"), None, True),
        (recipe("chmod -x ./scripts/a.sh"), None, True),
        (recipe("chmod -x scripts/./a.sh"), None, True),
        (recipe("chmod +x scripts/*.sh"), None, True),
        (recipe("chmod +x scripts/x.sh && echo ok"), None, True),
        (recipe("chmod +x scripts/x.sh>/dev/null"), None, True),
        (recipe('chmod +x "scripts/x.sh"'), None, True),
        (recipe("chmod +x $(F)"), None, True),
        (recipe("chmod +x /usr/local/bin/tool"), None, True),
        (recipe("@echo $(shell chmod +x scripts/x.sh)"), None, True),
        (recipe("chmod +x  scripts/x.sh"), None, True),
        # continuations and comments
        (recipe("chmod +x \\\n\t  scripts/x.sh"), None, True),
        (recipe("chmod -x scripts/x.s\\\n\th"), None, True),
        (recipe("# note \\\n\tchmod +x scripts/a.sh"), None, True),
        ("C := ls\\\nchmod\nt:\n\t@echo ok\n", None, True),
        ("C := chmod\nt:\n\t@echo ok\n", None, True),
        ("define C\nchmod +x scripts/x.sh\nendef\nt:\n\t@echo ok\n", None, True),
        # approvals
        (recipe(approved_line), None, True),
        (recipe(approved_line), f"{approved_line} && true\n", True),
        (recipe("@echo ok"), f"{approved_line}\n", True),
        # a carriage return: make does not break a line on it
        ("t:\n\t-chmod +x scripts/x.sh\r\tchmod 644 scripts/a.sh\n", None, True),
        ("t:\r\n\t@echo ok\r\n", None, True),
        # a make file that make reads ahead of Makefile, in any letter case; directives that change how recipes run
        (recipe("@echo ok"), None, True, {"GNUMAKEFILE": "t:\n\t@echo ok\n"}),
        (".ONESHELL:\nt:\n\t@chmod +x scripts/x.sh\n", None, True),
        ("  .ONESHELL:\nt:\n\t@echo ok\n", None, True),
        (".RECIPEPREFIX = >\nt:\n>@echo ok\n", None, True),
        ("all .ONESHELL:\nt:\n\t@echo ok\n", None, True),
        ("t .RECIPEPREFIX:\nt:\n\t@echo ok\n", None, True),
        ("override .RECIPEPREFIX = >\nt:\n\t@echo ok\n", None, True),
        ("all: .RECIPEPREFIX = >\nt:\n\t@echo ok\n", None, True),
        ("all: override .RECIPEPREFIX = >\nt:\n\t@echo ok\n", None, True),
        ("all \\\n.ONESHELL:\nt:\n\t@echo ok\n", None, True),
        ("all: \\\n\t.RECIPEPREFIX = >\nt:\n\t@echo ok\n", None, True),
        ("D := .ONESHELL\n$(D):\nt:\n\t@echo ok\n", None, True),
        (".ONE$(X)SHELL:\nt:\n\t@echo ok\n", None, True),
        ("$(eval .ONESHELL:)\nt:\n\t@echo ok\n", None, True),
        ("all: $(V) = >\nt:\n\t@echo ok\n", None, True),
        ("X := $(shell echo a)\nt:\n\t@echo ok\n", None, False),
        ("all: $(OBJS)\nt:\n\t@echo ok\n", None, False),
        ("all: FOO = bar\nt:\n\t@echo ok\n", None, False),
        (recipe("@echo $(X)"), None, False),
        (recipe("@$(eval .ONESHELL:)"), None, True),
        (recipe("@${eval .ONESHELL:}"), None, True),
        ("define F\n$(eval A := 1)\nendef\nt:\n\t@echo ok\n", None, True),
        ("X := $(eval A := 1)\nt:\n\t@echo ok\n", None, True),
        (recipe("# $(eval x)"), None, True),
        (recipe("@echo eval"), None, False),
        (recipe("@echo $(shell echo a)"), None, False),
        ("# $(eval x)\nt:\n\t@echo ok\n", None, False),
        ("# $(eval x) \\\nmore\nt:\n\t@echo ok\n", None, False),
        ("t:\n\t@$(ev\\\n\tal .ONESHELL:)\n", None, True),
        (recipe("@echo .ONESHELL: x"), None, False),
        ("# .ONESHELL: \\\nmore\nt:\n\t@echo ok\n", None, False),
        ("# .ONESHELL: is not set here\nt:\n\t@echo ok\n", None, False),
        ("X = .ONESHELL\nt:\n\t@echo ok\n", None, False),
        # trailing blanks on a plain line
        (recipe("chmod +x scripts/x.sh "), None, False),
        (recipe("@chmod +x scripts/x.sh\t"), None, False),
        (recipe("chmod +x scripts/a.sh "), None, True),
        (recipe("@echo ok"), None, True, {"GNUmakefile": "t:\n\t@echo ok\n"}),
        (recipe("chmod +x scripts/x.sh"), None, True, {"GNUmakefile": "t:\n\t@echo ok\n"}),
        # an allow file with no rows (an indented comment is a comment), and a comment after other text on a line
        (recipe("@echo ok"), "  # indented note\n\t# tab note\n", False),
        (recipe(approved_line), f"  # indented note\n  {approved_line}\n", False),
        (recipe("@echo ok"), "# no rows\n", False),
        (recipe("chmod +x scripts/x.sh"), "# no rows\n", False),
        (recipe("chmod +x scripts/a.sh"), "# no rows\n", True),
        (recipe(approved_line), "# no rows\n", True),
        ("X := 1 # chmod is set below\nt:\n\t@echo ok\n", None, True),
        (recipe("@echo ok # chmod +x scripts/a.sh"), None, True),
    ]
    # `makefile` beside `Makefile` can exist only on a case-sensitive file system.
    with tempfile.TemporaryDirectory() as probe:
        open(os.path.join(probe, "a"), "w").close()
        if not os.path.exists(os.path.join(probe, "A")):
            cases.append((recipe("@echo ok"), None, True, {"makefile": "t:\n\t@echo ok\n"}))
    tracked_files = [
        ("scripts/a.sh", "-x"),
        ("scripts/x.sh", "+x"),
        ("scripts/y.sh", "+x"),
        ("tools/run.sh", "+x"),
        ("Docs/Run.sh", "-x"),
        ("scripts/run_it.sh", "+x"),
        ("scripts/v2.sh", "+x"),
        ("scripts/NEW.sh", "+x"),
        ("scripts/new-file.sh", "+x"),
        ("scripts/c++.sh", "+x"),
        ("scripts/a@b.sh", "+x"),
        ("scripts/a,b.sh", "+x"),
        ("scripts/a%b.sh", "+x"),
        ("scripts/a=b.sh", "+x"),
        ("scripts/a:b.sh", "+x"),
        ("-R", "+x"),
        ("--recursive", "+x"),
        ("--", "+x"),
        ("scripts/h1.sh", "+x"),
        ("scripts/h2.sh", "-x"),
        ("scripts/h4.sh", "+x"),
        ("swap/run.sh", "+x"),
        ("scripts/swapfile.sh", "+x"),
    ]
    bad = 0
    # A hook may export these; the check must read the checkout's own index regardless.
    for name in AMBIENT_GIT:
        os.environ[name] = os.path.join(tempfile.gettempdir(), "test-script-modes-not-a-repository")
    env = clean_git_env()
    for makefile, allow, should_fail, *rest in cases:
        extra_files = rest[0] if rest else {}
        with tempfile.TemporaryDirectory() as root:
            run = lambda *a: subprocess.run(["git", "-C", root, *a], check=True, capture_output=True, env=env)
            run("init", "-q")
            for path, mode in tracked_files:
                os.makedirs(os.path.join(root, os.path.dirname(path)), exist_ok=True)
                with open(os.path.join(root, path), "w") as f:
                    f.write("#!/bin/sh\n")
                run("add", "--", path)
                run("update-index", f"--chmod={mode}", "--", path)
            for link, target in [
                ("scripts/link", "x.sh"),
                ("scripts/link-a", "a.sh"),
                ("scripts/dirlink", "../tools"),
                ("scripts/gone", "x.sh"),
            ]:
                os.symlink(target, os.path.join(root, link))
                run("add", link)
            os.remove(os.path.join(root, "scripts/gone"))
            # Tracked as regular files, replaced on disk by symlinks: the file's name is a tracked name, and the
            # shell follows the link out of it.
            shutil.rmtree(os.path.join(root, "swap"))
            os.symlink("tools", os.path.join(root, "swap"))
            os.remove(os.path.join(root, "scripts/swapfile.sh"))
            os.symlink("x.sh", os.path.join(root, "scripts/swapfile.sh"))
            # Symlinks git does not track, which the shell follows into tracked files.
            for link, target in [
                ("scripts/ulink", "x.sh"),
                ("scripts/ulink-a", "a.sh"),
                ("scripts/udir", "../tools"),
                (".git/hooks/pre-commit", "../../scripts/x.sh"),
                ("dotgit", ".git"),
            ]:
                os.symlink(target, os.path.join(root, link))
            with open(os.path.join(root, "scripts/untracked.sh"), "w") as f:
                f.write("#!/bin/sh\n")
            # Tracked, but not on disk.
            blob = run("hash-object", "-w", "scripts/x.sh").stdout.decode().strip()
            run("update-index", "--add", "--cacheinfo", f"100755,{blob},Missing/Ab.sh")
            # A tracked path that is not valid UTF-8, which no case names: reading the index must not crash on it.
            run("update-index", "--add", "--cacheinfo", f"100644,{blob},scripts/bad".encode() + b"\xff.sh")
            # Tracked as a symlink and as a submodule, though regular files on disk.
            for name, kind in (("symreg.sh", "120000"), ("subm.sh", "160000")):
                with open(os.path.join(root, "scripts", name), "w") as f:
                    f.write("#!/bin/sh\n")
                run("update-index", "--add", "--cacheinfo", f"{kind},{blob},scripts/{name}")
            # Hard links: tracked files that are one file on disk, and untracked names for them.
            os.remove(os.path.join(root, "scripts/h2.sh"))
            os.link(os.path.join(root, "scripts/h1.sh"), os.path.join(root, "scripts/h2.sh"))
            os.link(os.path.join(root, "scripts/h1.sh"), os.path.join(root, "scripts/h3.sh"))
            os.link(os.path.join(root, "scripts/h4.sh"), os.path.join(root, "scripts/h5.sh"))
            with open(os.path.join(root, "Makefile"), "w") as f:
                f.write(makefile)
            if allow is not None:
                with open(os.path.join(root, ALLOW), "w") as f:
                    f.write(allow)
            for name, content in extra_files.items():
                with open(os.path.join(root, name), "w") as f:
                    f.write(content)
            failed = bool(check(root)[0])
            ok = failed == should_fail
            bad += not ok
            shown = makefile.replace("t:\n\t", "", 1)
            print(f"{'ok  ' if ok else 'FAIL'} {'refused' if failed else 'passed '} {shown!r}")
    if bad:
        print(f"{bad} self-test case(s) wrong")
        sys.exit(1)
    print(f"script modes self-test: {len(cases)} cases ok")


if sys.argv[1:] == ["--self-test"]:
    self_test()
    sys.exit(0)
if sys.argv[1:]:
    print("usage: test-script-modes.sh [--self-test]", file=sys.stderr)
    sys.exit(2)
problems, notes = check(".")
for line in notes:
    print(f"note: {line}")
for line in problems:
    print(f"FAIL: {line}")
if problems:
    print(f"{len(problems)} script mode problem(s)")
    sys.exit(1)
print("script modes: ok")
PY
