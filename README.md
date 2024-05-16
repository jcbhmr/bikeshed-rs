# Bikeshed Rust

🦀 Bikeshed distributed as a Cargo package

<table align=center>

```sh
cargo install bikeshed
```

</table>

## Installation

You can install Bikeshed globally using Cargo:

```sh
cargo install bikeshed2
```

📛 This package is published on [crates.io](https://crates.io/) as `bikeshed2`. [`bikeshed` is already taken. 🙁](https://crates.io/crates/bikeshed)

ℹ This Rust CLI project provides standalone self-sufficient binaries that are [`cargo binstall`](https://github.com/cargo-bins/cargo-binstall)-able. You can find them on [the GitHub releases page](https://github.com/jcbhmr/bikeshed-builder/releases).

```sh
cargo binstall bikeshed2
```

## Usage

You can use the provided binary executable just as you normally would use `bikeshed` from any other installation method.

<details><summary><code>bikeshed --help</code></summary>

```
usage: bikeshed [-h] [--version] [-q] [-s] [-f] [-d] [-a] [--print {plain,console,markup,json}] [--die-on {everything,message,lint,warning,link-error,fatal,nothing}] [--die-when {early,late}] [--no-update] [--allow-nonlocal-files] [--allow-execute]
                {spec,echidna,watch,serve,update,issues-list,debug,refs,source,test,profile,template,wpt} ...

Bikeshed v4.1.6: Processes spec source files into valid HTML.

options:
  -h, --help            show this help message and exit
  --version             show program's version number and exit
  -q, --quiet           Silences one level of message, least-important first.
  -s, --silent          Shorthand for 'as many -q as you need to shut it up'
  -f, --force           Force the preprocessor to run to completion; fatal errors don't stop processing.
  -d, --dry-run         Prevents the processor from actually saving anything to disk, but otherwise fully runs.
  -a, --ascii-only      Force all Bikeshed messages to be ASCII-only.
  --print {plain,console,markup,json}
                        How Bikeshed formats its message output. Options are 'plain' (just text), 'console' (text with console color codes), 'markup' (XML), and 'json' (JSON stream). Defaults to 'console'.
  --die-on {everything,message,lint,warning,link-error,fatal,nothing}
                        Determines what sorts of errors cause Bikeshed to die (refuse to generate an output document). Default is 'fatal'; the -f flag is a shorthand for 'nothing'
  --die-when {early,late}
                        When a disallowed error should force Bikeshed to stop. 'early' causes it to stop immediately so you can deal with the first error; 'late' makes it process the entire document first and only stop at the end so you can see all the errors.
  --no-update           Skips checking if your data files are up-to-date.
  --allow-nonlocal-files
                        Allows Bikeshed to see/include files from folders higher than the one your source document is in.
  --allow-execute       Allow some features to execute arbitrary code from outside the Bikeshed codebase.

Subcommands:
  {spec,echidna,watch,serve,update,issues-list,debug,refs,source,test,profile,template,wpt}
    spec                Process a spec source file into a valid output file.
    echidna             Process a spec source file into a valid output file and publish it according to certain automatic protocols.
    watch               Process a spec source file into a valid output file, automatically rebuilding when it changes.
    serve               Identical to 'watch', but also serves the folder on localhost.
    update              Update supporting files (those in /spec-data).
    issues-list         Process a plain-text issues file into HTML. Call with no args to see an example input text.
    debug               Run various debugging commands.
    refs                Search Bikeshed's ref database.
    source              Tools for formatting the *source* document.
    test                Tools for running Bikeshed's testsuite.
    profile             Profiling Bikeshed. Needs graphviz, gprof2dot, and xdot installed.
    template            Outputs a skeleton .bs file for you to start with.
    wpt                 Tools for writing Web Platform Tests.
```

</details>
