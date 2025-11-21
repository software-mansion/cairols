# Cairo Language Server Maintenance

## Maintainers

All CairoLS maintainers are members of the **[@software-mansion/cairo-ls]** GitHub team.
Current project leader is **[@THenry14]**.

## Release procedure

In order to release a new version of `cairo-language-server` follow steps below.

1. Create a new branch, preferably denoting the next version of cairols to be released.
2. Bump `cairo-language-server` version in `Cargo.toml`.
3. Bump `cairo-lint` dependency to appropriate version.
4. Make sure all the `cairo-lang-*` dependencies are set to a version appropriate for your release. 
You can use the following command:
```bash
cargo xtask upgrade cairo <<VERSION>>
```
where <<VERSION>> is the appropriate version.

**NOTE:** The `patch` section in `Cargo.tom`l should be empty after doing this.
5. Push the changes, create a PR, verify the CI passes all checks. Have it merged to the main branch.
6. (Optional) If releasing for the first time, run:
```bash
cargo login
```
and follow terminal prompts to generate a token with at least publish-update permission.
7. Publish the crate using the following command:
```bash
cargo publish
```

OR (if using multiple tokens for multiple crates):

```bash
cargo publish --token <<token>>
```
8. Notify **[@software-mansion/scarb-maintainers]** team about the new `cairo-language-server` release.
9. After new scarb is released, wait for (or trigger a) [MAAT report](https://github.com/software-mansion/maat). 
This report should be studied closely to see if there are any regressions, unexpected problems or significant slowness of the LS.

It is also a good practice to test the new CairoLS release manually on some projects to examine the performance and see
if things are running rather smoothly and without any major problems. This can be done anytime between 5th and last steps.

[@software-mansion/scarb-maintainers]: https://github.com/orgs/software-mansion/teams/scarb-maintainers
[@software-mansion/cairo-ls]: https://github.com/orgs/software-mansion/teams/cairo-ls
[@THenry14]: https://github.com/THenry14
