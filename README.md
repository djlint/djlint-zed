# djLint for Zed

Lints and formats HTML templates with [djLint](https://djlint.com) in [Zed](https://zed.dev), through its language server. It works in HTML and in the languages of the template extensions: Django, Jinja2, Twig, Nunjucks, Liquid, Mustache, Tera, Angular and Go HTML templates.

## Installation

Install the extension from Zed's extensions page. It needs djLint 1.49 or newer, and looks for it in this order:

1. The `binary` setting below.
2. `djlint` on PATH.
3. [uv](https://docs.astral.sh/uv/), which runs djLint with no install.

## Usage

Zed doesn't turn on an extension's language server for a language by itself. Add `djlint` to the ones you want, and use it to format them:

```json
{
  "languages": {
    "Django": {
      "language_servers": ["djlint", "..."],
      "formatter": { "language_server": { "name": "djlint" } }
    }
  }
}
```

## Settings

djLint reads the project's [configuration](https://djlint.com/docs/configuration/) as usual. Settings from Zed win over it, and are described in the docs of the [language server](https://djlint.com/docs/language-server/#settings):

```json
{
  "lsp": {
    "djlint": {
      "binary": { "path": "/path/to/djlint", "arguments": ["--lsp"] },
      "settings": { "profile": "django", "ignore": ["H013"] }
    }
  }
}
```

## License

[MIT](LICENSE)
