# djLint for Zed

Lints and formats HTML templates with [djLint](https://djlint.com) in [Zed](https://zed.dev), through its language server. It works in HTML and in the languages of the template extensions: Django, Jinja2, HTML-Jinja, Twig, Nunjucks, Liquid, Handlebars, Mustache, Tera, Mako, Angular and Go HTML templates.

## Installation

Install it from Zed: open the command palette (`cmd-shift-p` on macOS, `ctrl-shift-p` elsewhere), run **zed: extensions**, search for djLint and click **Install**. Zed updates it by itself.

It needs djLint 1.49 or newer, and looks for it in this order:

1. The `binary` setting below.
2. `node_modules/.bin/djlint` in the project, which `npm i -D djlint` puts there. The [npm package](https://www.npmjs.com/package/djlint) comes with its own Python and runs on the `node` on PATH.
3. `djlint` on PATH, such as one from `uv tool install`, pip, pipx or `npm i -g`.
4. [uv](https://docs.astral.sh/uv/), which runs djLint with no install.

## Usage

Findings appear in these languages as soon as the extension is installed. To format with djLint, make it the formatter of the languages you want:

```json
{ "languages": { "Django": { "formatter": { "language_server": { "name": "djlint" } } } } }
```

To leave a language out, such as plain HTML, turn the server off for it:

```json
{ "languages": { "HTML": { "language_servers": ["!djlint", "..."] } } }
```

## Settings

djLint reads the project's [configuration](https://djlint.com/docs/configuration/) as usual. Settings from Zed win over it, and are described in the docs of the [language server](https://djlint.com/docs/integrations/#settings):

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
