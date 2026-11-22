# Releasing

Use [cargo-release](https://github.com/crate-ci/cargo-release) to prepare each
release. It updates Cargo.toml and Cargo.lock, moves the Unreleased notes into a
dated changelog entry, commits, tags, and pushes. The tag-triggered
[CD workflow](.github/workflows/cd.yml) verifies the release, publishes to
crates.io, and uploads the `parse_qemu_command` binaries to GitHub Releases.
Local publishing is disabled in [release.toml](release.toml) to keep one publisher.

## Version policy

For now, keep the existing `QEMU-version-N` convention. Choose an explicit version
every time: for example, `11.1.0-2` for another builder revision against the same
QEMU baseline, or `11.2.0-0` when intentionally advancing the baseline. Do not use
`cargo release patch`: it would remove the current numeric prerelease suffix.

Cargo treats `-N` as a [SemVer prerelease](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field),
even when we regard it as a normal release. Consumers should specify the version
explicitly, for example `cargo install qemu-command-builder --version 11.1.0-1`.
Longer term, independent crate SemVer versions with the supported QEMU baseline
documented separately would communicate Rust API compatibility more accurately.
Changing that policy should be an explicit decision before a future release.

The local QEMU audit records an RC baseline; it does not establish that a new QEMU
or crate version is ready to release. Review the baseline and public API changes
when choosing the next version.

## Prepare and preview

Install the tested tool version once:

```sh
cargo install cargo-release --version 1.1.5 --locked
```

1. Merge the intended changes and useful user-facing notes under `## [Unreleased]`
   in CHANGELOG.md. Use Added, Changed, Fixed, or other existing categories.
2. Check out `master`, pull with `git pull --ff-only`, and start with a clean working
   tree. Check that the new version has not already been published or tagged.
3. Run the same checks as CI:

   ```sh
   cargo fmt --all --check
   cargo clippy --locked --all-targets --all-features --workspace -- -D warnings
   cargo test --locked --all-features --workspace
   RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps --document-private-items --all-features --workspace --examples
   cargo package --locked
   ```

4. Preview with the chosen version (this example is not a release decision):

   ```sh
   cargo release 11.1.0-2
   ```

   Dry-run is the default. Review the version, changelog replacement, commit,
   tag, and push preview. See the upstream
   [command reference](https://github.com/crate-ci/cargo-release/blob/main/docs/reference.md).

## Publish

After reviewing the preview, run the same command with `--execute`:

```sh
cargo release 11.1.0-2 --execute
```

This creates the release commit and `v11.1.0-2` tag and pushes them to `origin`.
GitHub must allow the maintainer to push the release commit to `master`; follow
repository branch rules before running this command. CI requires an existing
`CARGO_REGISTRY_TOKEN` secret with permission to publish this crate. GitHub release
assets use the workflow's `GITHUB_TOKEN` with `contents: write`.

Watch the CD workflow. Its verification job checks that the tag matches the
manifest and changelog, then runs formatting, Clippy, tests, docs, and package
verification before publication or asset uploads. Confirm the published crate,
GitHub archives, and checksums. Each archive contains `parse_qemu_command` (or
`parse_qemu_command.exe` on Windows).

## Recover a partial release

Publication and binary uploads run independently after verification. If either
fails, inspect which jobs completed before retrying. Re-run only failed jobs;
do not re-run a successful crates.io publishing job, since published versions
are immutable. For a packaging-only failure after publication, fix the packaging
workflow and use a new crate revision if a source change is needed. Do not move
an existing published tag or reuse its version. Before publication, a failed
release can be corrected locally and previewed again; inspect local commits and
tags before attempting another `--execute`.
