# 👨‍🎨 ids: icons dev server

Small development server to preview svg icons added or modified in a git repository.

## Usage

To get a list of available commands and options, run:

```bash
$ ids --help
  ╭
 { } ids

usage
  ids [options] [args...]

ids (icons dev server) helps you visualize icons you modified
or added to a git repository during development

options
  --help, -h     show this help message and exit
  --version, -v  show version information
  --port, -v     set the port to run the server on (defaults to 8788)
  --host, -h     set the host to run the server on
  --no-spa       deactivate SPA (single page application) beahvior
  --no-ui        do not serve the UI
  --dir, -d      set the directory to run the command in (defaults to current)
```

To start the development server, run:

```bash
ids
```

This will start the server on port 8788 (default). You can now navigate to `http://localhost:8788`
to see a preview of all modified or added icons in the repository showing them on both light and
dark backgrounds.

The app will refresh automatically when you add or modify an icon.

## Notes

- Assumes light version of icons to have the same name as the dark version with a `_light` suffix.
- If a light version of an icon is not found, the dark version will be used on both backgrounds.
- ... and vice versa.

## TODO

- [ ] Create an off-canvas drawer when clicking on an icon to show it with an overlayed grid with
    different predefined adjustable sizes.
- [ ] Ability to configure the light/dark suffixes using a cli options.
