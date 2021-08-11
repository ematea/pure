# Pure

> [fish](https://fishshell.com/) prompt plugin

## Installation

Install with [Fisher](https://github.com/jorgebucaran/fisher):

```fish
fisher install ematea/pure
```

## Features

- The `pwd` and `git` information is displayed using the fish's build-in command `prompt_pwd` and `fish_git_prompt`. You can customize these as usual.
- `git` information is drawn asynchronously.
- Username and hostname is only displayed when you are logged in via SSH.

## Configuration

- As mentioned above, the `pwd` information is displayed using [prompt_pwd](https://fishshell.com/docs/current/cmds/prompt_pwd.html) and the `git` information using [fish_git_prompt](https://fishshell.com/docs/current/cmds/fish_git_prompt.html), so you can customize the colors and symbols of them according to the official documentation.
- Modify following variables to customize other symbols and colors in your `config.fish` file.

### Symbols

| Variable             | Type   | Description    | Default |
|----------------------|--------|----------------|---------|
| `pure_symbol_prompt` | string | Prompt symbol. | ❯       |

### Colors

| Variable                   | Type  | Description                                                  | Default |
|----------------------------|-------|--------------------------------------------------------------|---------|
| `pure_color_ssh`           | color | Color of ssh information section.                            | normal  |
| `pure_color_pwd`           | color | Color of the pwd section.                                    | normal  |
| `pure_color_duration`      | color | Color of the duration section.                               | normal  |
| `pure_color_prompt`        | color | Color of prompt symbol.                                      | normal  |
| `pure_color_prompt_failed` | color | Color of prompt symbol when command exit status was not `0`. | red     |

### Flags

| Variable                                  | Type                   | Description                           | Default |
|-------------------------------------------|------------------------|---------------------------------------|---------|
| `pure_remove_git_information_parenthesis` | string (true or false) | remove `()` of git information or not | false   |

