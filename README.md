# deezconfigs

[![license: GPL v3+](https://img.shields.io/badge/license-GPLv3+-blue)](https://www.gnu.org/licenses/gpl-3.0)
![GitHub Tag](https://img.shields.io/github/v/tag/qrichert/deezconfigs?sort=semver&filter=*.*.*&label=release)
[![crates.io](https://img.shields.io/crates/d/deezconfigs?logo=rust&logoColor=white&color=orange)](https://crates.io/crates/deezconfigs)

_Manage deez config files._

> [!CAUTION]
>
> Work In Progress.

Same idea as `GNU Stow` or `chezmoi`, but way simpler, requiring less
neuron activation to operate.

## Roadmap

- [x] CLI arguments parsing.
- [x] Basic `sync`.
- [x] Protect `$HOME` with a `.deez` file.
- [x] Add `--verbose` mode (use `-V` for version).
- [x] Use Git remote as `sync` root.
- [ ] Basic `link`.
- [ ] Basic `rsync`.
- [ ] Likely `list` (with up-to-date status for each file).
- [ ] Maybe `diff` (difference between source and target).
- [ ] Proper verbose `--help` section.
- [ ] Think about templating.

## Usage

`deez` will copy any file it finds under the `root` to a matching file
in `$HOME`.

- In `sync` mode, the files are copied and the target files overridden.
- `rsync` is the reverse of `sync`, copying from `$HOME` to `root`.
- In `link` mode, symlinks are created to the files in the root.

No special heuristics are planned outside of templates (find a less
intimidating name for that) and variables/secrets.

`deez` requires a `.deez` file in the config root (or it will ask for
confirmation), to prevent yourself from ruining the `$HOME` directory
if ran on the wrong root.

Maybe add `deez diff`, that diffs the two versions (using the `diff`
executable, or settable through an env variable), nothing fancy or
homemade. Maybe there's a crate for this?

```console
$ tree
.
├── .config
│   ├── fish
│   │   └── config.fish
│   ├── ghostty
│   │   ├── config
│   │   └── ssh.txt
│   └── nvim
│       └── init.vim
├── .deez
├── .gitconfig
└── .tmux.conf
```

```console
$ deez --help
deez sync|rsync|link [<root>]

$ deez sync
Copies the files to the $HOME directory.
Also creates any missing directories.

$ deez sync git://git@github.com:qrichert/configs.git
Sync directly from Git remote.

$ deez link
Creates symlinks to the files in $HOME.
Also creates any missing directories.

$ deez sync somedir
Will treat `somedir` as the root instead of using `cwd`.
```

- Respects `.gitignore` (thanks to the `ignore` crate) (TODO: test
  this).

### Templating (idea/TODO)

Sometimes you don't want to replace the whole config file, just a part
of it. For example, in `config.fish` you may have generic `fish` config,
but you also want a variable per-machine section.

With `deez` you can achieve this by creating a template section in the
target config file (won't work in `link` mode).

```
# <%deez-template%>

This section will be overridden/updated by the `deez` config file.

# </%deez-template%>

Outside of the template won't be touched.
```

- Only one template opening is allowed per file.
- Only one template closing is allowed per file.
- If there's an opening there must be a closing.
- If there's a closing there must be an opening.
