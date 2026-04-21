<!--
SPDX-FileCopyrightText: © 2024 Matt Williams <matt.williams@bristol.ac.uk>
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Making a release

Releases are completely automated by GitHub Actions:
1. Make sure the [changelog](./CHANGELOG.md) is up to date.
   A release cannot be made if there is nothing in the changelog in the `Unreleased` section.
2. Go the the [Release](https://github.com/isambard-sc/clifton/actions/workflows/release.yml) workflow page.
3. Click "Run workflow" in the top right.
4. Make sure the `master` branch is selected.
5. In the box below, type `patch`, `minor` or `major`, depending on the [SemVer level](https://semver.org) of release to make.
6. Press the "Run workflow" button.

This will kick off a series of chained workflows, culminating in a new release appearing on the [Releases page](https://github.com/isambard-sc/clifton/releases).

## Releasing on WinGet

Due to [fine-grained GitHub tokens not supporting opening PRs on public repos](https://github.com/github/roadmap/issues/600), we cannot automate the submission to WinGet.
This means that after a release, a manual step is required.

This could be done by manually submitting a PR to add a new entry to `manifests/c/clifton/clifton` in the https://github.com/microsoft/winget-pkgs repo, but there is a tool, `komac`, to automate this.

First, create a personal access token (classic) with an expiration of 7 days (this limit is imposed by the `isambard-sc` organisation) and with the `public_repo` scope.
Run `komac token update` and paste in the token.

```shell-session
VERSION=0.3.0 komac update clifton.clifton --version "${VERSION}" --urls "https://github.com/isambard-sc/clifton/releases/download/${VERSION}/clifton-windows-x86_64.zip"
```
