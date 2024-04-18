# Contributing

## Setup

For development instruction see [Authoring Zed Extensions](https://github.com/zed-industries/extensions/blob/main/AUTHORING_EXTENSIONS.md).

## Development

### Install the required tools

We use [Just](https://just.systems/man/en/) to run scripts and tasks, to make our life easier.

You can install `just` using cargo:

```shell
cargo install just
```

But we **highly recommend
** to [install it using an OS package manager](https://github.com/casey/just#packages),  so you won't need to prefix every command with `cargo`.

Once installed, run the following command install the required tools:

```shell
just install-tools
```

1. Clone this repository.
1. Open Zed
1. Open the command palette <kbd>Ctrl</kbd>/<kbd title="Cmd">⌘</kbd>+<kbd title="Shift">⇧</kbd>+<kbd>P</kbd>
1. Run the `zed: install dev extensions` command.
1. Select the directory of this repo.

If you make changes to the Rust code and you require to reload the extension,  you can open the "Extensions" tab by running the command `zed: extensions`, choose the `"Installed"`, seek the current extension and click the `"Rebuild"` label.

#### Logs

Zed will print logs in the following directory: `~/Library/Logs/Zed/Zed.log`
