# toad-scaffold

Project scaffolding logic for the
[Primatif Toad](https://github.com/Primatif/Primatif_Toad) ecosystem.

## What It Does

`toad-scaffold` provides **project creation and initialization**. It generates
new project directories with the correct structure, configuration files, and
git setup based on configurable templates.

- **Project Creation** — `create_project()` scaffolds a new project directory
  with stack-appropriate files, git initialization, and optional editor launch.
- **Configuration** — `ProjectConfig` defines the scaffolding parameters:
  project name, target directory, stack type, and editor preference.
- **Editor Integration** — `open_in_editor()` launches the newly created
  project in the user's preferred editor.

## Role in the Ecosystem

`toad-scaffold` is the creation layer. It depends on `toad-core` (data models)
and `toad-git` (git initialization). It is consumed by the CLI (`toad new`
command).

```text
toad-core ──┐
            ├── toad-scaffold ── bin/toad (toad new)
toad-git  ──┘
```

## License

MIT
