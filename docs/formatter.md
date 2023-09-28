# The Ruff Formatter

The Ruff formatter is an extremely fast Python code formatter designed as a drop-in replacement for
[Black](https://pypi.org/project/black/), available as part of the `ruff` CLI (as of Ruff v0.0.289).

## `ruff format`

`ruff format` is the primary entrypoint for the formatter. It accepts a list of files or
directories, and formats all discovered Python files:

```shell
ruff format .                 # Format all files in the current directory.
ruff format /path/to/file.py  # Format a single file.
```

Similar to Black, running `ruff format /path/to/file.py` will format the given file or directory
in-place, while `ruff format --check /path/to/file.py` will avoid writing any formatted files back,
instead exiting with a non-zero status code if any files are not already formatted.

For the full list of supported options, run `ruff format --help`.

## Philosophy

The goal of the Ruff Formatter is _not_ to innovate on code style, but rather, to innovate on
performance, and provide a unified toolchain across Ruff's linter, formatter, and any and all
future tools.

As such, the formatter is designed as a drop-in replacement for [Black](https://github.com/psf/black),
but with an excessive focus on performance and direct integration with Ruff. Given Black's
popularity within the Python ecosystem, targeting Black compatibility ensures that formatter
adoption is minimally disruptive for the vast majority of projects.

Specifically, the formatter is intended to emit near-identical output when run over existing
Black-formatted code. When run over extensive Black-formatted projects like Django and Zulip, > 99.9%
of lines are formatted identically. (See: [Black compatibility](#black-compatibility).)

Given this focus on Black compatibility, the formatter thus adheres to [Black's (stable) code style](https://black.readthedocs.io/en/stable/the_black_code_style/current_style.html),
which aims for "consistency, generality, readability and reducing git diffs". To give you a sense
for the enforced code style, here's an example:

```python
# Input
def _make_ssl_transport(
    rawsock, protocol, sslcontext, waiter=None,
    *, server_side=False, server_hostname=None,
    extra=None, server=None,
    ssl_handshake_timeout=None,
    call_connection_made=True):
    '''Make an SSL transport.'''
    if waiter is None:
      waiter = Future(loop=loop)

    if extra is None:
      extra = {}

    ...

# Ruff
def _make_ssl_transport(
    rawsock,
    protocol,
    sslcontext,
    waiter=None,
    *,
    server_side=False,
    server_hostname=None,
    extra=None,
    server=None,
    ssl_handshake_timeout=None,
    call_connection_made=True,
):
    """Make an SSL transport."""
    if waiter is None:
        waiter = Future(loop=loop)

    if extra is None:
        extra = {}

    ...
```

Like Black, the Ruff Formatter does _not_ support extensive code style configuration; however,
unlike Black, it _does_ support configuring the desired quote style, indent style, line endings,
and more. (See: [_Configuration_](#configuration).)

While the formatter is designed to be a drop-in replacement for Black, it is not intended to be
used interchangeably with Black on an ongoing basis, as the formatter _does_ differ from
Black in a few conscious ways (see: [intentional deviations](#intentional-deviations)). In general,
deviations are limited to cases in which Ruff's behavior was deemed more consistent, or
significantly simpler to support (with negligible end-user impact) given the differences in the
underlying implementations between Black and Ruff.

Going forward, the Ruff Formatter will support Black's preview style under Ruff's own
[preview](preview.md) mode.

## Configuration

The Ruff Formatter exposes a small set of configuration options, some of which are also supported
by Black (like line width), some of which are unique to Ruff (like quote and indentation style).

For example, to configure the formatter to use double single quotes, a line width of 100, and
tab indentation, add the following to your `pyproject.toml`:

```toml
[tool.ruff.format]
line-length = 100
quote-style = "single"
indent-style = "tab"
```

For more on configuring Ruff via `pyproject.toml`, see [_Configuring Ruff_](configuration.md).

Given the focus on Black compatibility (and unlike formatters like [YAPF](https://github.com/google/yapf)),
Ruff does not currently expose any configuration options to modify core formatting behavior outside
of these trivia-related settings.

## Format suppression

Like Black, Ruff supports `# fmt: on`, `# fmt: off`, and `# fmt: skip` pragma comments, which can
be used to temporarily disable formatting for a given code block.

`# fmt: on` and `# fmt: off` comments are enforced at the statement level:

```python
# fmt: off
not_formatted=3
also_not_formatted=4
# fmt:on
```

As such, adding `# fmt: on` and `# fmt: off` comments within expressions will have no effect. In
the following example, both list entries will be formatted, despite the `# fmt: off`:

```python
[
    # fmt: off
    '1',
    # fmt: on
    '2',
]
```

Instead, apply the `# fmt: off` comment to the entire statement:

```python
# fmt: off
[
    '1',
    '2',
]
```

`# fmt: skip` comments suppress formatting for a preceding statement, case header, decorator,
function definition, or class definition:

```python
if True:
    pass
elif False: # fmt: skip
    pass

@Test
@Test2 # fmt: off
def test(): ...

a = [1, 2, 3, 4, 5] # fmt: off

def test(a, b, c, d, e, f) -> int: # fmt: skip
    pass
```

Like Black, Ruff will _also_ recognize [YAPF](https://github.com/google/yapf)'s `# yapf: disable` and `# yapf: enable` pragma
comments, which are treated equivalently to `# fmt: off` and `# fmt: on`, respectively.

## Conflicting lint rules

The Ruff Formatter is designed to be used alongside the Ruff Linter. However, the linter includes
some rules that, when enabled, can cause conflicts with the formatter, leading to unexpected
behavior.

When using Ruff as a formatter, we recommend disabling the following rules:

- [`line-too-long`](rules/line-too-long.md)
- [`single-line-implicit-string-concatenation`](rules/single-line-implicit-string-concatenation.md)
- [`missing-trailing-comma`](rules/missing-trailing-comma.md)
- [`prohibited-trailing-comma`](rules/prohibited-trailing-comma.md)
- [`bad-quotes-inline-string`](rules/bad-quotes-inline-string.md)
- [`bad-quotes-multiline-string`](rules/bad-quotes-multiline-string.md)
- [`bad-quotes-docstring`](rules/bad-quotes-docstring.md)
- [`avoidable-escaped-quote`](rules/avoidable-escaped-quote.md)

Similarly, we recommend disabling the following isort settings, which are incompatible with the
formatter's treatment of import statements when set to non-default values:

- [`lines-after-imports`](https://docs.astral.sh/ruff/settings/#isort-lines-after-imports)
- [`lines-between-types`](https://docs.astral.sh/ruff/settings/#isort-lines-between-types)

## Exit codes

`ruff format` exits with the following status codes:

- `0` if no files were formatted.
- `1` if any files were formatted.
- `2` if Ruff terminates abnormally due to invalid configuration, invalid CLI options, or an
    internal error.

`ruff format --check` uses the same exit codes, such that an exit code of `1` indicates that a
file _would_ be formatted if `--check` were not specified, while an exit code of `0` indicates that
no files would be formatted.

## Black compatibility

The formatter is designed to be a drop-in replacement for [Black](https://github.com/psf/black).

Specifically, the formatter is intended to emit near-identical output when run over Black-formatted
code. When run over extensive Black-formatted projects like Django and Zulip, > 99.9% of lines
are formatted identically. When migrating an existing project from Black to Ruff, you should expect
to see a few differences on the margins, but the vast majority of your code should be unchanged.

When run over _non_-Black-formatted code, the formatter makes some different decisions than Black,
and so more deviations should be expected, especially around the treatment of end-of-line comments.

If you identify deviations in your project, spot-check them against the [intentional deviations](#intentional-deviations)
enumerated below, as well as the [unintentional deviations](https://github.com/astral-sh/ruff/issues?q=is%3Aopen+is%3Aissue+label%3Aformatter)
filed in the issue tracker. If you've identified a new deviation, please [file an issue](https://github.com/astral-sh/ruff/issues/new).

### Preview style

Black gates formatting changes behind a [`preview`](https://black.readthedocs.io/en/stable/the_black_code_style/future_style.html#preview-style)
flag. The formatter does not yet support Black's preview style, though the intention is to support
it within the coming months.

### Intentional deviations

This section enumerates the known, intentional deviations between the Ruff formatter and Black's
stable style. (Unintentional deviations are tracked in the [issue tracker](https://github.com/astral-sh/ruff/issues?q=is%3Aopen+is%3Aissue+label%3Aformatter).)

<h4>Trailing end-of-line comments</h4>

Black's priority is to fit an entire statement on a line, even if it contains end-of-line comments.
In such cases, Black collapses the statement, and moves the comment to the end of the collapsed
statement:

```python
# Input
while (
    cond1  # almost always true
    and cond2  # almost never true
):
    print("Do something")

# Black
while cond1 and cond2:  # almost always true  # almost never true
    print("Do something")
```

Ruff, like [Prettier](https://prettier.io/), expands any statement that contains trailing
end-of-line comments. For example, Ruff would avoid collapsing the `while` test in the snippet
above. This ensures that the comments remain close to their original position and retain their
original intent, at the cost of retaining additional vertical space.

This deviation only impacts unformatted code, in that Ruff's output should not deviate for code that
has already been formatted by Black.

<h4>Pragma comments are ignored when computing line width</h4>

Pragma comments (`# type`, `# noqa`, `# pyright`, `# pylint`, etc.) are ignored when computing the width of a line.
This prevents Ruff from moving pragma comments around, thereby modifying their meaning and behavior:

See Ruff's [pragma comment handling proposal](https://github.com/astral-sh/ruff/discussions/6670)
for details.

This is similar to [Pyink](https://github.com/google/pyink) but a deviation from Black. Black avoids
splitting any lines that contain a `# type` comment ([#997](https://github.com/psf/black/issues/997)),
but otherwise avoids special-casing pragma comments.

As Ruff expands trailing end-of-line comments, Ruff will also avoid moving pragma comments in cases
like the following, where moving the `# noqa` to the end of the line causes it to suppress errors
on both `first()` and `second()`:

```python
# Input
[
    first(),  # noqa
    second()
]

# Black
[first(), second()]  # noqa

# Ruff
[
    first(),  # noqa
    second(),
]
```

<h4>Line width vs. line length</h4>

Ruff uses the Unicode width of a line to determine if a line fits. Black's stable style uses
character width, while Black's preview style uses Unicode width for strings ([#3445](https://github.com/psf/black/pull/3445)),
and character width for all other tokens. Ruff's behavior is closer to Black's preview style than
Black's stable style, although Ruff _also_ uses Unicode width for identifiers and comments.

<h4>Walruses in slice expressions</h4>

Black avoids inserting space around `:=` operators within slices. For example, the following adheres
to Black stable style:

```python
# Input
x[y:=1]

# Black
x[y:=1]
```

Ruff will instead add space around the `:=` operator:

```python
# Input
x[y:=1]

# Ruff
x[y := 1]
```

This will likely be incorporated into Black's preview style ([#3823](https://github.com/psf/black/pull/3823)).

<h4><code>global</code> and <code>nonlocal</code> names are broken across multiple lines by continuations</h4>

If a `global` or `nonlocal` statement includes multiple names, and exceeds the configured line
width, Ruff will break them across multiple lines using continuations:

```python
# Input
global analyze_featuremap_layer, analyze_featuremapcompression_layer, analyze_latencies_post, analyze_motions_layer, analyze_size_model

# Ruff
global \
    analyze_featuremap_layer, \
    analyze_featuremapcompression_layer, \
    analyze_latencies_post, \
    analyze_motions_layer, \
    analyze_size_model
```

<h4>Newlines are inserted after all class docstrings</h4>

Black typically enforces a single newline after a class docstring. However, it does not apply such
formatting if the docstring is single-quoted rather than triple-quoted, while Ruff enforces a
single newline in both cases:

```python
# Input
class IntFromGeom(GEOSFuncFactory):
    "Argument is a geometry, return type is an integer."
    argtypes = [GEOM_PTR]
    restype = c_int
    errcheck = staticmethod(check_minus_one)

# Black
class IntFromGeom(GEOSFuncFactory):
    "Argument is a geometry, return type is an integer."
    argtypes = [GEOM_PTR]
    restype = c_int
    errcheck = staticmethod(check_minus_one)

# Ruff
class IntFromGeom(GEOSFuncFactory):
    "Argument is a geometry, return type is an integer."

    argtypes = [GEOM_PTR]
    restype = c_int
    errcheck = staticmethod(check_minus_one)
```

<h4>Trailing own-line comments on imports are not moved to the next line</h4>

Black enforces a single empty line between an import and a trailing own-line comment. Ruff leaves
such comments in-place:

```python
# Input
import os
# comment

import sys

# Black
import os

# comment

import sys

# Ruff
import os
# comment

import sys
```

<h4>Parentheses around awaited collections are not preserved</h4>

Black preserves parentheses around awaited collections:

```python
await ([1, 2, 3])
```

Ruff will instead remove them:

```python
await [1, 2, 3]
```

This is more consistent to the formatting of other awaited expressions: Ruff and Black both
remove parentheses around, e.g., `await (1)`, only retaining them when syntactically required,
as in, e.g., `await (x := 1)`.

<h4>Implicit string concatenations in attribute accesses</h4>

Given the following unformatted code:

```python
print("aaaaaaaaaaaaaaaa" "aaaaaaaaaaaaaaaa".format(bbbbbbbbbbbbbbbbbb + bbbbbbbbbbbbbbbbbb))
```

Internally, Black's logic will first expand the outermost `print` call:

```python
print(
    "aaaaaaaaaaaaaaaa" "aaaaaaaaaaaaaaaa".format(bbbbbbbbbbbbbbbbbb + bbbbbbbbbbbbbbbbbb)
)
```

Since the argument is _still_ too long, Black will then split on the operator with the highest split
precedence. In this case, Black splits on the implicit string concatenation, to produce the
following Black-formatted code:

```python
print(
    "aaaaaaaaaaaaaaaa"
    "aaaaaaaaaaaaaaaa".format(bbbbbbbbbbbbbbbbbb + bbbbbbbbbbbbbbbbbb)
)
```

Ruff gives implicit concatenations a "lower" priority when breaking lines. As a result, Ruff
would instead format the above as:

```python
print(
    "aaaaaaaaaaaaaaaa" "aaaaaaaaaaaaaaaa".format(
        bbbbbbbbbbbbbbbbbb + bbbbbbbbbbbbbbbbbb
    )
)
```

In general, Black splits implicit string concatenations over multiple lines more often than Ruff,
even if those concatenations _can_ fit on a single line. Ruff instead avoids splitting such
concatenations unless doing so is necessary to fit within the configured line width.

<h4>Own-line comments on expressions don't cause the expression to expand</h4>

Given an expression like:

```python
(
    # A comment in the middle
    some_example_var and some_example_var not in some_example_var
)
```

Black associates the comment with `some_example_var`, thus splitting it over two lines:

```python
(
    # A comment in the middle
    some_example_var
    and some_example_var not in some_example_var
)
```

Ruff will instead associate the comment with the entire boolean expression, thus preserving the
initial formatting:

```python
(
    # A comment in the middle
    some_example_var and some_example_var not in some_example_var
)
```

<h4>Tuples are parenthesized when expanded</h4>

Ruff tends towards parenthesizing tuples (with a few exceptions), while Black tends to remove tuple
parentheses more often.

In particular, Ruff will always insert parentheses around tuples that expand over multiple lines:

```python
# Input
(a, b), (c, d,)

# Black
(a, b), (
    c,
    d,
)

# Ruff
(
    (a, b),
    (
        c,
        c,
    ),
)
```

There's one exception here. In `for` loops, both Ruff and Black will avoid inserting unnecessary
parentheses:

```python
# Input
for a, f(b,) in c:
    pass

# Black
for a, f(
    b,
) in c:
    pass

# Ruff
for a, f(
    b,
) in c:
    pass
```

<h4>Single-element tuples are always parenthesized</h4>

Ruff always inserts parentheses around single-element tuples, while Black will omit them in some
cases:

```python
# Input
(a, b),

# Black
(a, b),

# Ruff
((a, b),)
```

Adding parentheses around single-element tuples adds visual distinction and helps avoid "accidental"
tuples created by extraneous trailing commas (see, e.g., [#17181](https://github.com/django/django/pull/17181)).

<h4>Trailing commas are inserted when expanding a function definition with a single argument</h4>

When a function definition with a single argument is expanded over multiple lines, Black
will add a trailing comma in some cases, depending on whether the argument includes a type
annotation and/or a default value.

For example, Black will add a trailing comma to the first and second function definitions below,
but not the third:

```python
def func(
    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
) -> None:
    ...


def func(
    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa=1,
) -> None:
    ...


def func(
    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: Argument(
        "network_messages.pickle",
        help="The path of the pickle file that will contain the network messages",
    ) = 1
) -> None:
    ...
```

Ruff will instead insert a trailing comma in all such cases for consistency.

<h4>Parentheses around call-chain assignment values are not preserved</h4>

Given:

```python
def update_emission_strength():
    (
        get_rgbw_emission_node_tree(self)
        .nodes["Emission"]
        .inputs["Strength"]
        .default_value
    ) = (self.emission_strength * 2)
```

Black will preserve the parentheses in `(self.emission_strength * 2)`, whereas Ruff will remove
them.

Both Black and Ruff remove such parentheses in simpler assignments, like:

```python
# Input
def update_emission_strength():
    value = (self.emission_strength * 2)

# Black
def update_emission_strength():
    value = self.emission_strength * 2

# Ruff
def update_emission_strength():
    value = self.emission_strength * 2
```

<h4>Type annotations may be parenthesized when expanded</h4>

Black will avoid parenthesizing type annotations in an annotated assignment, while Ruff will insert
parentheses in some cases.

For example:

```python
# Black
StartElementHandler: Callable[[str, dict[str, str]], Any] | Callable[[str, list[str]], Any] | Callable[
    [str, dict[str, str], list[str]], Any
] | None

# Ruff
StartElementHandler: (
    Callable[[str, dict[str, str]], Any]
    | Callable[[str, list[str]], Any]
    | Callable[[str, dict[str, str], list[str]], Any]
    | None
)
```
