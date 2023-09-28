# The Ruff Linter

The Ruff Linter is an extremely fast Python linter designed as a drop-in replacement for [Flake8](https://pypi.org/project/flake8/)
(plus dozens of plugins), [isort](https://pypi.org/project/isort/), [pydocstyle](https://pypi.org/project/pydocstyle/),
[pyupgrade](https://pypi.org/project/pyupgrade/), [autoflake](https://pypi.org/project/autoflake/),
and more.

## `ruff check`

`ruff check` is the primary entrypoint for the Ruff linter. It accepts a list of files or
directories, and lints all discovered Python files, optionally fixing any fixable errors:

```shell
ruff check .          # Lint all files in the current directory.
ruff check . --fix    # Lint all files in the current directory, and fix any fixable errors.
ruff check . --watch  # Lint all files in the current directory, and re-lint on change.
```

For the full list of supported options, run `ruff check --help`.

## Rule selection

The set of enabled rules is controlled via the [`select`](settings.md#select) and [`ignore`](settings.md#ignore)
settings, along with the [`extend-select`](settings.md#extend-select) and [`extend-ignore`](settings.md#extend-ignore)
modifiers.

Ruff's linter mirrors Flake8's rule code system, in which each rule code consists of a one-to-three
letter prefix, followed by three digits (e.g., `F401`). The prefix indicates that "source" of the rule
(e.g., `F` for Pyflakes, `E` for pycodestyle, `ANN` for flake8-annotations).

Rule selectors like [`select`](settings.md#select) and [`ignore`](settings.md#ignore) accept either
a full rule code (e.g., `F401`) or any valid prefix (e.g., `F`). For example, given the following
`pyproject.toml` file:

```toml
[tool.ruff.lint]
select = ["E", "F"]
ignore = ["F401"]
```

Ruff would enable all rules with the `E` (pycodestyle) or `F` (PyFlakes) prefix, with the exception
of `F401`. (For more on configuring Ruff via `pyproject.toml`, see [_Configuring Ruff_](configuration.md).)

As a special-case, Ruff also supports the `ALL` code, which enables all rules. Note that some
pydocstyle rules conflict (e.g., `D203` and `D211`) as they represent alternative docstring
formats. Ruff will automatically disable any conflicting rules when `ALL` is enabled.

If you're wondering how to configure Ruff, here are some **recommended guidelines**:

- Prefer `select` and `ignore` over `extend-select` and `extend-ignore`, to make your rule set
    explicit.
- Use `ALL` with discretion. Enabling `ALL` will implicitly enable new rules whenever you upgrade.
- Start with a small set of rules (`select = ["E", "F"]`) and add a category at-a-time. For example,
    you might consider expanding to `select = ["E", "F", "B"]` to enable the popular flake8-bugbear
    extension.
- By default, Ruff's fixes are aggressive. If you find that it's too aggressive for your liking,
    consider turning off fixes for specific rules or categories (see [_FAQ_](faq.md#ruff-tried-to-fix-something--but-it-broke-my-code-whats-going-on)).

For example, a configuration that enables some of the most popular rules (without being too
pedantic) might look like the following:

```toml
[tool.ruff.lint]
select = [
  # pycodestyle
  "E",
  # Pyflakes
  "F",
  # pyupgrade
  "UP",
  # flake8-bugbear
  "B",
  # flake8-simplify
  "SIM",
  # isort
  "I",
]
```

To resolve the enabled rule set, Ruff may need to reconcile `select` and `ignore` from a variety
of sources, including the current `pyproject.toml`, any inherited `pyproject.toml` files, and the
CLI (e.g., `--select`).

In those scenarios, Ruff uses the "highest-priority" `select` as the basis for the rule set, and
then applies any `extend-select`, `ignore`, and `extend-ignore` adjustments. CLI options are given
higher priority than `pyproject.toml` options, and the current `pyproject.toml` file is given higher
priority than any inherited `pyproject.toml` files.

For example, given the following `pyproject.toml` file:

```toml
[tool.ruff.lint]
select = ["E", "F"]
ignore = ["F401"]
```

Running `ruff check --select F401` would result in Ruff enforcing `F401`, and no other rules.

Running `ruff check --extend-select B` would result in Ruff enforcing the `E`, `F`, and `B` rules,
with the exception of `F401`.

## Automatic fixes

Ruff supports automatic fixes for a variety of lint errors. For example, Ruff can remove unused
imports, reformat docstrings, rewrite type annotations to use the latest PEP syntax, and more.

To enable automatic fixes, pass the `--fix` flag to `ruff check`:

```shell
ruff check . --fix
```

By default, Ruff will fix all violations for which automatic fixes are available. (To determine
whether a rule supports automatic fixing, see [_Rules_](rules.md).) To limit the set of rules
that Ruff should attempt to fix, use the [`fixable`](settings.md#fixable) and [`unfixable`](settings.md#unfixable)
settings, along with their [`extend-fixable`](settings.md#extend-fixable) and [`extend-unfixable`](settings.md#extend-unfixable)
variants.

For example, the following configuration would enable fixes for all rules except
[`unused-imports`](rules/unused-import.md) (`F401`):

```toml
[tool.ruff.lint]
fixable = ["ALL"]
unfixable = ["F401"]
```

## Error suppression

Ruff supports several mechanisms for suppressing lint errors, be they false positives or
permissible violations.

To omit a lint rule entirely, add it to the "ignore" list via the [`ignore`](settings.md#ignore)
or [`extend-ignore`](settings.md#extend-ignore) settings, either on the command-line
or in your `pyproject.toml` or `ruff.toml` file.

To suppress a violation inline, Ruff uses a `noqa` system similar to [Flake8](https://flake8.pycqa.org/en/3.1.1/user/ignoring-errors.html).
To ignore an individual violation, add `# noqa: {code}` to the end of the line, like so:

```python
# Ignore F841.
x = 1  # noqa: F841

# Ignore E741 and F841.
i = 1  # noqa: E741, F841

# Ignore _all_ violations.
x = 1  # noqa
```

For multi-line strings (like docstrings), the `noqa` directive should come at the end of the string
(after the closing triple quote), and will apply to the entire string, like so:

```python
"""Lorem ipsum dolor sit amet.

Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor.
"""  # noqa: E501
```

To ignore all violations across an entire file, add `# ruff: noqa` to any line in the file, like so:

```python
# ruff: noqa
```

To ignore a specific rule across an entire file, add `# ruff: noqa: {code}` to any line in the file,
like so:

```python
# ruff: noqa: F841
```

Or see the [`per-file-ignores`](settings.md#per-file-ignores) setting, which enables the same
functionality from within your `pyproject.toml` or `ruff.toml` file.

Note that Ruff will also respect Flake8's `# flake8: noqa` directive, and will treat it as
equivalent to `# ruff: noqa`.

### Detecting unused suppression comments

Ruff implements a special rule, [`unused-noqa`](https://docs.astral.sh/ruff/rules/unused-noqa/),
under the `RUF100` code, to enforce that your `noqa` directives are "valid", in that the violations
they _say_ they ignore are actually being triggered on that line (and thus suppressed). To flag
unused `noqa` directives, run: `ruff check /path/to/file.py --extend-select RUF100`.

Ruff can also _remove_ any unused `noqa` directives via its fix functionality. To remove any
unused `noqa` directives, run: `ruff check /path/to/file.py --extend-select RUF100 --fix`.

### Inserting necessary suppression comments

Ruff can _automatically add_ `noqa` directives to all lines that contain violations, which is
useful when migrating a new codebase to Ruff. To automatically add `noqa` directives to all
relevant lines (with the appropriate rule codes), run: `ruff check /path/to/file.py --add-noqa`.

### Action comments

Ruff respects isort's [action comments](https://pycqa.github.io/isort/docs/configuration/action_comments.html)
(`# isort: skip_file`, `# isort: on`, `# isort: off`, `# isort: skip`, and `# isort: split`), which
enable selectively enabling and disabling import sorting for blocks of code and other inline
configuration.

Ruff will also respect variants of these action comments with a `# ruff:` prefix
(e.g., `# ruff: isort: skip_file`, `# ruff: isort: on`, and so on). These variants more clearly
convey that the action comment is intended for Ruff, but are functionally equivalent to the
isort variants.

See the [isort documentation](https://pycqa.github.io/isort/docs/configuration/action_comments.html)
for more.

## Exit codes

By default, `ruff check` exits with the following status codes:

- `0` if no violations were found, or if all present violations were fixed automatically.
- `1` if violations were found.
- `2` if Ruff terminates abnormally due to invalid configuration, invalid CLI options, or an
    internal error.

This convention mirrors that of tools like ESLint, Prettier, and RuboCop.

`ruff check` supports two command-line flags that alter its exit code behavior:

- `--exit-zero` will cause Ruff to exit with a status code of `0` even if violations were found.
    Note that Ruff will still exit with a status code of `2` if it terminates abnormally.
- `--exit-non-zero-on-fix` will cause Ruff to exit with a status code of `1` if violations were
    found, _even if_ all such violations were fixed automatically. Note that the use of
    `--exit-non-zero-on-fix` can result in a non-zero exit code even if no violations remain after
    fixing.
