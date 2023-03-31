# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.4.0](https://github.com/ianwelsh/heimdallr/compare/0.3.0..0.4.0) - 2023-03-31
#### Bug Fixes
- how do you toml? - ([7d6989f](https://github.com/ianwelsh/heimdallr/commit/7d6989f4dacf44622a9512092286321a1d4f0922)) - Ian Welsh
- replace prettytable with comfy-table - ([24a27ab](https://github.com/ianwelsh/heimdallr/commit/24a27abe24524ca09790d58e43b3ea0900fbec6c)) - Ian Welsh
- default to ~/.config/heimdallr.toml for mac and linux; drop windows - ([65df5ad](https://github.com/ianwelsh/heimdallr/commit/65df5adbbe5e496784abbef302a77c71b082db06)) - Ian Welsh
#### Features
- spawn an ssh process instead of just printing ssh string - ([849a4b8](https://github.com/ianwelsh/heimdallr/commit/849a4b863f22ba3829153bc350a9e7d02920f9c2)) - Ian Welsh
#### Miscellaneous Chores
- clippy is too picky - ([0c0a04c](https://github.com/ianwelsh/heimdallr/commit/0c0a04cd9683f996ec247f4e39c0c2adc72091fb)) - Ian Welsh
- bump rust edition - ([b72f3bb](https://github.com/ianwelsh/heimdallr/commit/b72f3bbc8b0030d1d35c9dfa499c8984155b45d0)) - Ian Welsh
- more release prep - ([dbfa992](https://github.com/ianwelsh/heimdallr/commit/dbfa9925b25f15908d0b27c9089cb731f5e05cd0)) - Ian Welsh
- preparing to release from my fork - ([c37ebac](https://github.com/ianwelsh/heimdallr/commit/c37ebacb04c2147979a368b537ba841477a9aa6b)) - Ian Welsh
- update self_update (fixes "will be rejected by a future Rust" warning) - ([83505f0](https://github.com/ianwelsh/heimdallr/commit/83505f0dabd4b3c8f59008aafae43fd3e72906c6)) - Ian Welsh
- update config package (fixes "will be rejected by a future Rust" warning) - ([cef16d0](https://github.com/ianwelsh/heimdallr/commit/cef16d06ec0dec0f29addaf3eaecd8a07640e1cd)) - Ian Welsh
- remove unused dirs package - ([59bfb4f](https://github.com/ianwelsh/heimdallr/commit/59bfb4fd1e446739701923126a62e5f2dea24d9d)) - Ian Welsh

- - -

## 0.3.0 - 2021-10-20


### Features

[a337c7](https://github.com/keelerm84/heimdallr/commit/a337c75ec5f72c860f66f3bc7cb71f05171a6bea) - Add StrictHostKeyChecking=no to ssh (#4) - Beshoy Girgis

[53c422](https://github.com/keelerm84/heimdallr/commit/53c422fbf65443269050a5356325d6a6ddc27a2e) - Add support for configurable region (#3) - Beshoy Girgis


- - -
## 0.2.0 - 2021-05-13


### Features

[4d119f](https://github.com/keelerm84/heimdallr/commit/4d119fdcdb35a39b82724e5d05b628571fcb6e6e) - Introduce configuration file (#2) - Matthew M. Keeler


### Documentation

[b6f89a](https://github.com/keelerm84/heimdallr/commit/b6f89ab37757c9096b6e2ce005fdc7410686217d) - Remove completed tasks from roadmap - Matthew M. Keeler


- - -
## 0.1.0 - 2021-05-11


### Documentation

[735057](https://github.com/keelerm84/heimdallr/commit/73505753ecbdfaa0bc667cfd025e08001afd07f4) - Add GitHub CODEOWNERS file - Matthew M. Keeler

[2f8a2d](https://github.com/keelerm84/heimdallr/commit/2f8a2d24e88c3efa5bf40a4ef4090b06314086ac) - Add disclaimer - Matthew M. Keeler

[818285](https://github.com/keelerm84/heimdallr/commit/81828545fa623c65b3aa24819d000c18f564e10e) - Add roadmap - Matthew M. Keeler

[88dbcb](https://github.com/keelerm84/heimdallr/commit/88dbcbd64188d83eb0f2f7371338c925015dbd01) - Add README and license - Matthew M. Keeler


### Build system

[d9ed6c](https://github.com/keelerm84/heimdallr/commit/d9ed6c53ffd0f803032973392e5e70d4463dde80) - Add release process - Matthew M. Keeler

[d74871](https://github.com/keelerm84/heimdallr/commit/d74871a21ac86bffd76ede1325742a3b8a8eef8b) - Adding support for cocogitto - Matthew M. Keeler

[dfd59b](https://github.com/keelerm84/heimdallr/commit/dfd59b365b91092a57061c33745f92354146c051) - Disable publish option - Matthew M. Keeler


### Refactoring

[c761d2](https://github.com/keelerm84/heimdallr/commit/c761d2a39659dbc7f5382a74c0ba8d49b60e55b0) - Fix cargo clippy --all-targets --all-features - Matthew M. Keeler

[44d7b9](https://github.com/keelerm84/heimdallr/commit/44d7b96225e08b10d5d169e2bc96fe6aa32270c6) - Extract arn_to_id function - Matthew M. Keeler

[1644a0](https://github.com/keelerm84/heimdallr/commit/1644a07a61761a8eb6211c479a52ca34190a8d56) - Extract list work into ui layer - Matthew M. Keeler

[2a8791](https://github.com/keelerm84/heimdallr/commit/2a8791e575a91db35e530ac345f4eb0659181b29) - Extract connect work into ui layer - Matthew M. Keeler

[2a8c79](https://github.com/keelerm84/heimdallr/commit/2a8c79ee306ea755117d4d2511825a36a8bc4e56) - Reorganize to layered architecture - Matthew M. Keeler


### Continuous Integration

[34050a](https://github.com/keelerm84/heimdallr/commit/34050a822ee95d77f013c380d6d1e19eb7f4ae17) - Build and publish binaries on tag push - Matthew M. Keeler

[9e21e6](https://github.com/keelerm84/heimdallr/commit/9e21e68bfb71c17cb619371290cce0e4a8501386) - Add GitHub action support - Matthew M. Keeler


### Bug Fixes

[8a8ba5](https://github.com/keelerm84/heimdallr/commit/8a8ba5966ebd8aa6eadd814fe4443f806869a9c1) - Associate multiple task arns with container instance arns (#1) - Matthew M. Keeler

[feb3f9](https://github.com/keelerm84/heimdallr/commit/feb3f950266a4f72a55bc893aace4909cdabfc7f) - Improve display for list with 0 instances - Matthew M. Keeler


### Features

[aabcb7](https://github.com/keelerm84/heimdallr/commit/aabcb76194a60d29ef81f3702c12b92a9f1af2c5) - Add self-updating sub command - Matthew M. Keeler

[402ec2](https://github.com/keelerm84/heimdallr/commit/402ec271696143cea85100022d4fb230750e300e) - Add missing ssh parameters - Matthew M. Keeler

[73df60](https://github.com/keelerm84/heimdallr/commit/73df603694b7fe60eecced5fe132a36abb6644db) - Hook up profile flag - Matthew M. Keeler

[dcab37](https://github.com/keelerm84/heimdallr/commit/dcab37cd347be7e343908042837bb90e35022af2) - Add connection support for other patterns - Matthew M. Keeler

[4c50c0](https://github.com/keelerm84/heimdallr/commit/4c50c0f5f42859a2e40f88f79311c67f8a8443a2) - Implement grant and revoke commands - Matthew M. Keeler

[f18f43](https://github.com/keelerm84/heimdallr/commit/f18f43f2a1821adbda3f74f936d5d44553f977cb) - Implement list command - Matthew M. Keeler

[09d9b2](https://github.com/keelerm84/heimdallr/commit/09d9b253f2ebf08acbcf3723331d8bc4c6b80015) - Add structopt structure for future flags - Matthew M. Keeler


- - -

This changelog was generated by [cocogitto](https://github.com/oknozor/cocogitto).