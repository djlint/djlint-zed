<!--
Pull requests are squash-merged, so write the title in the Conventional Commits
style: `fix: ...`, `feat: ...`, `docs: ...`, `refactor: ...`, `test: ...`, `chore: ...`.

CI runs the linters and the build. To catch failures before you push, run:

uvx prek run --all-files
cargo +nightly fmt --check
cargo +beta clippy --target wasm32-wasip2 --
-->

## Summary

<!-- What does this change, and why? -->

## Related issues

<!-- `Closes #123` closes the issue on merge. Remove this section if there are none. -->

## Testing

<!-- How did you check that this works? Installing it with "zed: install dev extension" is the usual way. -->

## Checklist

- [ ] `README.md` matches any change to the settings or the supported languages.
