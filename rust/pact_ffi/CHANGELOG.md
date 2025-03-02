To generate the log, run `git log --pretty='* %h - %s (%an, %ad)' TAGNAME..HEAD .` replacing TAGNAME and HEAD as appropriate.

# 0.3.3 - Bug fixes + Support publishing results from webhook calls

* 9a6c846f - chore: Upgrade pact_matching to 0.12.9 (Ronald Holshausen, Fri Jun 10 15:46:07 2022 +1000)
* b3f98a2c - chore: Upgrade pact_verifier to 0.13.8 (Ronald Holshausen, Tue Jun 7 11:07:24 2022 +1000)
* 18118e82 - feat: add retries to the provider state change calls #197 (Ronald Holshausen, Tue Jun 7 09:10:23 2022 +1000)
* 23c0c593 - bump version to 0.3.3 (Ronald Holshausen, Mon May 30 14:31:20 2022 +1000)

# 0.3.2 - Bugfix Release

* 42dcd525 - chore: Disable ANSI escape codes in logs as Pact .Net is unable to deal with them (Ronald Holshausen, Mon May 30 13:16:05 2022 +1000)
* 61fc3771 - chore: Upgrade pact_verifier to 0.13.7 (Ronald Holshausen, Mon May 30 12:21:12 2022 +1000)
* f42026d5 - chore: Upgrade pact_mock_server to 0.9.1 (Ronald Holshausen, Mon May 30 12:09:06 2022 +1000)
* bcddbcfb - chore: Upgrade pact_matching to 0.12.8 (Ronald Holshausen, Mon May 30 11:52:26 2022 +1000)
* 80256458 - chore: Upgrade pact-plugin-driver to 0.1.8 (Ronald Holshausen, Mon May 30 11:36:54 2022 +1000)
* 873f0c93 - fix(ffi): resources were not freed correctly when the mock server is provided by a plugin (Ronald Holshausen, Mon May 30 11:05:20 2022 +1000)
* e32caf8d - bump version to 0.3.2 (Ronald Holshausen, Tue May 24 15:55:23 2022 +1000)

# 0.3.1 - Bugfix Release

* 797d1cce - fix(ffi): plugin data was not merged into the Pact file correctly (Ronald Holshausen, Tue May 24 14:13:25 2022 +1000)
* a78f2a1d - fix(ffi): pactffi_create_mock_server_for_transport was returning the wrong status for invalid address (Ronald Holshausen, Mon May 23 16:56:32 2022 +1000)
* bf70164f - bump version to 0.3.1 (Ronald Holshausen, Mon May 23 15:33:59 2022 +1000)

# 0.3.0 - Support mock servers from plugins

* 5cd2ae5a - feat: add pactffi_create_mock_server_for_transport function (Ronald Holshausen, Fri May 20 16:09:36 2022 +1000)
* d9b9fe72 - chore: Upgrade pact-plugin-driver to 0.1.7 (Ronald Holshausen, Fri May 20 15:56:23 2022 +1000)
* 2b24b52b - chore(ffi): update mock server function docs (Ronald Holshausen, Tue May 17 13:45:44 2022 +1000)
* bb6f6f47 - chore(CI): fix examples with cmake on Windows (Ronald Holshausen, Tue May 17 13:22:40 2022 +1000)
* ec49b971 - chore(CI): fix examples with cmake on Windows (Ronald Holshausen, Mon May 16 18:03:43 2022 +1000)
* 0b2ac979 - chore(CI): fix examples with cmake on Windows (Ronald Holshausen, Mon May 16 17:40:38 2022 +1000)
* 2ae32295 - chore(ffi): fix examples with cmake on Windows (Ronald Holshausen, Mon May 16 17:07:25 2022 +1000)
* 6d76df16 - chore(CI): copy OSX dylib to example build dir (Ronald Holshausen, Mon May 16 16:37:33 2022 +1000)
* 888e6586 - chore: cleanup compiler warnings (Ronald Holshausen, Mon May 16 16:28:46 2022 +1000)
* 1307dde0 - fix(ffi): OSX CMake file had the wring filename (Ronald Holshausen, Mon May 16 16:04:51 2022 +1000)
* e1ddffc3 - feat(ffi): open log files in append mode (Ronald Holshausen, Mon May 16 15:31:16 2022 +1000)
* d76e417c - chore: re-enable the FFI examples in CI (Ronald Holshausen, Mon May 16 14:29:22 2022 +1000)
* b14fb2b1 - refactor: convert the FFI logging functions to setup a tracing subscriber (Ronald Holshausen, Mon May 16 14:18:22 2022 +1000)
* f8471bb7 - chore: switch from log crate to tracing crate (Ronald Holshausen, Fri May 13 13:47:18 2022 +1000)
* 1c97e1e6 - chore: Upgrade depedent crates (Ronald Holshausen, Thu May 12 11:04:34 2022 +1000)
* 1a973502 - chore: bump minor version of FFI crate (Ronald Holshausen, Thu May 12 11:01:48 2022 +1000)
* ee9d6bab - chore: Upgrade pact_verifier to 0.13.6 (Ronald Holshausen, Wed May 11 17:40:15 2022 +1000)
* f6b942da - chore: Upgrade pact_mock_server to 0.8.11 (Ronald Holshausen, Wed May 11 17:00:46 2022 +1000)
* 08f28e4a - chore: Upgrade pact_matching to 0.12.7 (Ronald Holshausen, Wed May 11 15:57:36 2022 +1000)
* 37bfc5de - chore: Upgrade pact-plugin-driver to 0.1.6 (Ronald Holshausen, Wed May 11 11:56:23 2022 +1000)
* 020b5715 - chore: upgrade pact_models to 0.4.1 (Ronald Holshausen, Wed May 11 11:36:57 2022 +1000)
* e8d62b79 - bump version to 0.2.7 (Ronald Holshausen, Wed Apr 27 16:46:23 2022 +1000)

# 0.2.6 - Maintenance Release

* 14a010a9 - chore: Upgrade pact_verifier to 0.13.5 (Ronald Holshausen, Wed Apr 27 15:21:15 2022 +1000)
* 563ae9fc - chore: Upgrade pact_mock_server to 0.8.10 (Ronald Holshausen, Wed Apr 27 15:06:50 2022 +1000)
* bcae77b4 - chore: upgrade pact_matching to 0.12.6 (Ronald Holshausen, Wed Apr 27 14:29:26 2022 +1000)
* dba7252e - chore: Upgrade pact-plugin-driver to 0.1.5 (Ronald Holshausen, Tue Apr 26 13:56:22 2022 +1000)
* 688e49e7 - chore: Upgrade pact-plugin-driver to 0.1.4 (Ronald Holshausen, Fri Apr 22 14:47:01 2022 +1000)
* cdf72b05 - feat: forward provider details to plugin when verifying (Ronald Holshausen, Fri Apr 22 14:12:34 2022 +1000)
* 2395143a - feat: forward verification to plugin for transports provided by the plugin (Ronald Holshausen, Fri Apr 22 12:02:05 2022 +1000)
* 2eeaccf4 - bump version to 0.2.6 (Ronald Holshausen, Tue Apr 19 13:59:46 2022 +1000)

# 0.2.5 - Maintenance Release

* d41e2440 - fix(ffi): correct race condition in pactffi_using_plugin (Ronald Holshausen, Wed Apr 13 16:51:36 2022 +1000)
* 136c8a82 - chore: Upgrade pact_verifier to 0.13.4 (Ronald Holshausen, Wed Apr 13 16:06:02 2022 +1000)
* 1e8ae855 - chore: Upgrade pact_mock_server to 0.8.9 (Ronald Holshausen, Wed Apr 13 15:49:03 2022 +1000)
* 0df06dd2 - chore: Upgrade pact_matching to 0.12.5 (Ronald Holshausen, Wed Apr 13 15:38:49 2022 +1000)
* d043f6c7 - chore: upgrade pact_models to 0.3.3 (Ronald Holshausen, Wed Apr 13 15:24:33 2022 +1000)
* eee09ba6 - chore: Upgrade pact-plugin-driver to 0.1.3 (Ronald Holshausen, Wed Apr 13 14:07:36 2022 +1000)
* 73ae0ef0 - fix: Upgrade reqwest to 0.11.10 to resolve #156 (Ronald Holshausen, Wed Apr 13 13:31:55 2022 +1000)
* ffeca2e2 - chore: update to the latest plugin driver (Ronald Holshausen, Wed Apr 13 13:08:25 2022 +1000)
* efaba75b - chore: Update release for FFI to use the correct nightly Rust for cbindgen (Ronald Holshausen, Wed Apr 13 10:40:34 2022 +1000)
* 610490ab - chore: trying to get the ffi cmake build working (Ronald Holshausen, Tue Apr 12 18:09:47 2022 +1000)
* e13eb80d - chore: Update ci-build.sh to use the same nightly Rust as build (Ronald Holshausen, Tue Apr 12 17:49:21 2022 +1000)
* 776265ee - chore: bump pact_verifier to 0.13.3 (Ronald Holshausen, Thu Mar 24 15:05:01 2022 +1100)
* 89027c87 - chore: update pact_matching (0.12.4) and pact_mock_server (0.8.8) (Ronald Holshausen, Thu Mar 24 14:09:45 2022 +1100)
* 9baf03a9 - chore: use the published version of the plugin driver (Ronald Holshausen, Thu Mar 24 13:36:01 2022 +1100)
* 42b1a461 - Merge branch 'master' into feat/plugin-mock-server (Ronald Holshausen, Mon Mar 21 16:01:33 2022 +1100)
* 345b0011 - feat: support mock servers provided from plugins (Ronald Holshausen, Mon Mar 21 15:59:46 2022 +1100)
* 63b63358 - bump version to 0.2.5 (Matt Fellows, Mon Mar 21 11:29:45 2022 +1100)

# 0.2.4 - Bugfix Release

* 13f7c36f - fix: xml response matching rules (Matt Fellows, Wed Mar 9 17:07:56 2022 +1100)
* c5b96ebb - chore: need musl-tools om release build (Ronald Holshausen, Fri Mar 4 17:15:43 2022 +1100)
* 01b7adb9 - bump version to 0.2.4 (Ronald Holshausen, Fri Mar 4 16:46:18 2022 +1100)
* b67292db - update changelog for release 0.2.3 (Ronald Holshausen, Fri Mar 4 16:42:52 2022 +1100)
* 16fbe7cf - feat: add musl target to the release build #185 (Ronald Holshausen, Fri Mar 4 16:23:39 2022 +1100)
* b6433500 - chore: upgrade pact_verifier to 0.13.2 (Ronald Holshausen, Fri Mar 4 14:49:18 2022 +1100)
* 5a4a8a1c - chore: update pact_mock_server to 0.8.7 (Ronald Holshausen, Fri Mar 4 14:24:23 2022 +1100)
* 8894fdfd - chore: update pact_matching to 0.12.3 (Ronald Holshausen, Fri Mar 4 14:09:17 2022 +1100)
* 8e864502 - chore: update all dependencies (Ronald Holshausen, Fri Mar 4 13:29:59 2022 +1100)
* f52c3625 - feat: add for custom headers to the HTTP client used by the verifier #182 (Ronald Holshausen, Mon Feb 28 14:38:00 2022 +1100)
* 74bd4531 - feat: add support for custom headers with the verifier FFI calls #182 (Ronald Holshausen, Mon Feb 28 13:58:46 2022 +1100)
* c6d553e0 - bump version to 0.2.3 (Ronald Holshausen, Mon Feb 14 13:45:19 2022 +1100)

# 0.2.3 - Support Custom headers + Date-Time expression parser

* 16fbe7cf - feat: add musl target to the release build #185 (Ronald Holshausen, Fri Mar 4 16:23:39 2022 +1100)
* b6433500 - chore: upgrade pact_verifier to 0.13.2 (Ronald Holshausen, Fri Mar 4 14:49:18 2022 +1100)
* 5a4a8a1c - chore: update pact_mock_server to 0.8.7 (Ronald Holshausen, Fri Mar 4 14:24:23 2022 +1100)
* 8894fdfd - chore: update pact_matching to 0.12.3 (Ronald Holshausen, Fri Mar 4 14:09:17 2022 +1100)
* 8e864502 - chore: update all dependencies (Ronald Holshausen, Fri Mar 4 13:29:59 2022 +1100)
* f52c3625 - feat: add for custom headers to the HTTP client used by the verifier #182 (Ronald Holshausen, Mon Feb 28 14:38:00 2022 +1100)
* 74bd4531 - feat: add support for custom headers with the verifier FFI calls #182 (Ronald Holshausen, Mon Feb 28 13:58:46 2022 +1100)
* c6d553e0 - bump version to 0.2.3 (Ronald Holshausen, Mon Feb 14 13:45:19 2022 +1100)

# 0.2.2 - Bugfix Release

* 76889087 - fix(pact-ffi): intermediate JSON - add test for JSON with decimal matcher #179 (Ronald Holshausen, Mon Feb 14 13:04:16 2022 +1100)
* b10453c3 - fix(pact-ffi): intermediate JSON - type matcher paths were being incorrectly allocated to children #179 (Ronald Holshausen, Mon Feb 14 12:45:43 2022 +1100)
* 1555c682 - bump version to 0.2.2 (Ronald Holshausen, Thu Feb 3 14:12:46 2022 +1100)

# 0.2.1 - add option to strip ANSI control codes from verifier output

* 506add91 - chore: bump pact_verifier version (Ronald Holshausen, Thu Feb 3 13:54:45 2022 +1100)
* cc872209 - chore: add non-windows init ansi support function (Ronald Holshausen, Thu Feb 3 13:22:51 2022 +1100)
* 7311e022 - feat(FFI): add option to strip ANSI control codes from verifier output (Ronald Holshausen, Thu Feb 3 12:29:02 2022 +1100)
* c18e1ccc - chore: ANSI support function was missing pactffi prefix (Ronald Holshausen, Thu Feb 3 11:12:45 2022 +1100)
* fbfd072f - feat(FFI): add an explicit function to enable ANSI terminal support on Windows (Ronald Holshausen, Thu Feb 3 11:11:30 2022 +1100)
* 07806b05 - bump version to 0.2.1 (Ronald Holshausen, Mon Jan 31 14:28:33 2022 +1100)

# 0.2.0 - Bugfixes + FFI functions to return the verifier output and results

* 1d95f3cf - chore: Bump minor version of Pact FFI lib (Ronald Holshausen, Mon Jan 31 13:58:42 2022 +1100)
* 739cb7b8 - chore: fix missing import on Windows (Ronald Holshausen, Mon Jan 31 11:16:55 2022 +1100)
* 5ecf70a7 - feat: enable ANSI console output on Windows (Ronald Holshausen, Mon Jan 31 11:02:03 2022 +1100)
* c676e821 - feat: add FFI functions to return the verifier output and results (Ronald Holshausen, Fri Jan 28 15:40:17 2022 +1100)
* bf152233 - feat: Capture all the results from the verification process (Ronald Holshausen, Fri Jan 28 11:28:38 2022 +1100)
* 5f148cdd - feat: capture all the output from the verifier (Ronald Holshausen, Thu Jan 27 16:08:02 2022 +1100)
* f5aa34ea - Merge pull request #175 from pact-foundation/feat/fix-provider-timeout-value-validation (Ronald Holshausen, Thu Jan 27 13:41:56 2022 +1100)
* c58a2fb7 - Merge pull request #174 from adamrodger/feat/provider-name (Ronald Holshausen, Thu Jan 27 13:39:26 2022 +1100)
* 0ef3fb98 - fix: provider request timeout should be > 16bit integers. Fixes https://github.com/pact-foundation/pact-js/issues/761 (Matt Fellows, Wed Jan 26 22:12:35 2022 +1100)
* 753c9599 - feat(ffi)!: Remove the need to repeat the provider name in verifier FFI (Adam Rodger, Wed Jan 26 10:17:23 2022 +0000)
* 8bee40b0 - feat(ffi)!: Separate verification and publishing options (Adam Rodger, Tue Jan 25 16:31:29 2022 +0000)
* bef310b2 - bump version to 0.1.7 (Ronald Holshausen, Mon Jan 17 17:20:07 2022 +1100)

# 0.1.6 - Maintenance Release

* 0c200ea5 - chore: Upgrade pact verifier crate to 0.12.4 (Ronald Holshausen, Mon Jan 17 17:07:18 2022 +1100)
* 10c9b842 - chore: Upgrade pact_mock_server to 0.8.6 (Ronald Holshausen, Mon Jan 17 16:57:31 2022 +1100)
* 5e4c68ef - chore: update pact matching to 0.12.2 (Ronald Holshausen, Mon Jan 17 16:29:21 2022 +1100)
* 80b241c5 - chore: Upgrade plugin driver crate to 0.0.17 (Ronald Holshausen, Mon Jan 17 11:22:48 2022 +1100)
* 4f1ecff2 - chore: Upgrade pact-models to 0.2.7 (Ronald Holshausen, Mon Jan 17 10:53:26 2022 +1100)
* 63ab0d2d - fix: generators in process_object (Matt Fellows, Sat Jan 15 23:21:34 2022 +1100)
* c2089645 - fix: log crate version must be fixed across all crates (including plugin driver) (Ronald Holshausen, Fri Jan 14 16:10:50 2022 +1100)
* 255d6eba - bump version to 0.1.6 (Ronald Holshausen, Tue Jan 4 10:59:38 2022 +1100)

# 0.1.5 - Maintenance Release

* 7dbdd456 - chore: update test log crate (Ronald Holshausen, Tue Jan 4 10:46:46 2022 +1100)
* 1b16e30a - chore: test-env-log has been renamed to test-log (Ronald Holshausen, Tue Jan 4 10:43:51 2022 +1100)
* 1cafd00a - fix: drop(from_raw(ptr))` if you intend to drop the `CString` (Ronald Holshausen, Tue Jan 4 10:39:16 2022 +1100)
* fe22ae3a - fix: expected opaque type, found enum `Result` (Ronald Holshausen, Tue Jan 4 10:26:22 2022 +1100)
* 213d1459 - fix: add a small delay after loading plugins via FFI to resolve a race condition (Ronald Holshausen, Tue Jan 4 09:56:33 2022 +1100)
* 9c2810ad - chore: Upgrade pact-plugin-driver to 0.0.15 (Ronald Holshausen, Fri Dec 31 15:12:56 2021 +1100)
* 0a6e7d9d - refactor: Convert MatchingContext to a trait and use DocPath instead of string slices (Ronald Holshausen, Wed Dec 29 14:24:39 2021 +1100)
* 4d088317 - chore: Update pact_mock_server crate to 0.8.4 (Ronald Holshausen, Thu Dec 23 13:24:15 2021 +1100)
* 52bc1735 - chore: update pact_matching crate to 0.11.5 (Ronald Holshausen, Thu Dec 23 13:12:08 2021 +1100)
* 5479a634 - chore: Update pact_models (0.2.4) and pact-plugin-driver (0.0.14) (Ronald Holshausen, Thu Dec 23 12:57:02 2021 +1100)
* fc0a8360 - chore: update pact_matching to 0.11.4 (Ronald Holshausen, Mon Dec 20 12:19:36 2021 +1100)
* 8911d5b0 - chore: update to latest plugin driver crate (metrics fixes) (Ronald Holshausen, Mon Dec 20 12:11:35 2021 +1100)
* 9153cc5b - bump version to 0.1.5 (Ronald Holshausen, Wed Dec 15 16:44:22 2021 +1100)

# 0.1.4 - Maintenance Release

* a1d03b95 - chore: update dependent pact crates (Ronald Holshausen, Wed Dec 15 16:34:39 2021 +1100)
* f8042d6b - feat: add metrics event for provider verification (Ronald Holshausen, Tue Dec 14 17:29:44 2021 +1100)
* 4f1ba7d9 - chore: update to the latest plugin driver (Ronald Holshausen, Tue Dec 14 13:55:02 2021 +1100)
* 2f97c25f - bump version to 0.1.4 (Ronald Holshausen, Thu Dec 2 13:21:24 2021 +1100)

# 0.1.3 - Bugfix Release

* 4184e562 - chore(pact_ffi): upgrade to latest models, matching and verifier crates (Ronald Holshausen, Thu Dec 2 13:13:37 2021 +1100)
* d43b1847 - Merge pull request #164 from tienvx/feat-filter-info (Ronald Holshausen, Fri Nov 19 11:38:41 2021 +1100)
* 41e69a22 - feat: allow set filter info (tienvx, Thu Nov 18 08:56:36 2021 +0700)
* 7c561f2a - feat: allow set consumer version selectors (tienvx, Thu Nov 18 00:12:31 2021 +0700)
* 260deb70 - fix: support specifying matching_branch in verifications (Matt Fellows, Wed Nov 17 17:47:37 2021 +1100)
* 86ea5779 - chore: fix FFI release build (Ronald Holshausen, Wed Nov 17 15:52:14 2021 +1100)
* 5480733f - bump version to 0.1.3 (Ronald Holshausen, Wed Nov 17 15:20:29 2021 +1100)

# 0.1.2 - Bugfix Release

* 631167fa - chore: update to latest mock server crate (Ronald Holshausen, Wed Nov 17 15:13:32 2021 +1100)
* 87e7f11e - chore: remove note from pactffi_write_pact_file (Ronald Holshausen, Wed Nov 17 14:55:22 2021 +1100)
* 5d4a09c6 - feat: store the pact specification version with the mock server (Ronald Holshausen, Wed Nov 17 14:46:56 2021 +1100)
* 4ccc5d02 - chore: update doc comment on pactffi_write_pact_file (Ronald Holshausen, Wed Nov 17 14:04:59 2021 +1100)
* 675506e1 - feat: add pactffi_pact_handle_write_file which knows about the spec version (Ronald Holshausen, Wed Nov 17 13:58:45 2021 +1100)
* 09f3b888 - refactor: make the pact handle types opaque (Ronald Holshausen, Wed Nov 17 13:27:06 2021 +1100)
* aff4d301 - fix: FFI always detects + stores JSON bodies as plain text (Matt Fellows, Tue Nov 16 23:02:12 2021 +1100)
* fc5be202 - fix: update to latest driver crate (Ronald Holshausen, Tue Nov 16 16:19:02 2021 +1100)
* e4a445ba - fix: race condition when shutting down plugin via FFI (Ronald Holshausen, Tue Nov 16 16:01:18 2021 +1100)
* f3c5e7c1 - bump version to 0.1.2 (Ronald Holshausen, Tue Nov 16 14:04:54 2021 +1100)

# 0.1.1 - Support V4 synchronous messages + protobuf plugin

* 5d974c4a - chore: update to latest models and plugin driver crates (Ronald Holshausen, Tue Nov 16 11:56:53 2021 +1100)
* 19beb0ea - feat(plugins): add support for synch messages via FFI (Ronald Holshausen, Tue Nov 16 10:06:07 2021 +1100)
* df23ba3d - fix: allow multiple consumer version selectors (Matt Fellows, Mon Nov 15 14:28:04 2021 +1100)
* 7c150c8b - feat(plugins): Support message tests via FFI that use plugins (Ronald Holshausen, Wed Nov 10 17:03:49 2021 +1100)
* 20643590 - feat(plugins): add plugin support to FFI functions (Ronald Holshausen, Tue Nov 9 16:06:01 2021 +1100)
* 62f7d36c - refactor: moved the message consumer FFI functions to the handles module (Ronald Holshausen, Mon Nov 8 17:42:26 2021 +1100)
* 0cb367d9 - refactor: moved the HTTP consumer FFI functions to the handles module (Ronald Holshausen, Mon Nov 8 17:25:16 2021 +1100)
* 2027537d - refactor: update FFI to use V4 models internally (Ronald Holshausen, Mon Nov 8 16:44:39 2021 +1100)
* e1ff90c7 - bump version to 0.1.1 (Ronald Holshausen, Thu Nov 4 17:23:32 2021 +1100)

# 0.1.0 - Pact V4 release

* 59e21413 - feat: Pact V4 release (Ronald Holshausen, Thu Nov 4 16:54:56 2021 +1100)
* 400a1231 - chore: drop beta from pact_verifier version (Ronald Holshausen, Thu Nov 4 15:56:22 2021 +1100)
* fc4580b8 - chore: drop beta from pact_mock_server version (Ronald Holshausen, Thu Nov 4 15:28:51 2021 +1100)
* bd2bd0ec - chore: drop beta from pact_matching version (Ronald Holshausen, Wed Nov 3 13:28:35 2021 +1100)
* 296b4370 - chore: update project to Rust 2021 edition (Ronald Holshausen, Fri Oct 22 10:44:48 2021 +1100)
* a561f883 - chore: use the non-beta models crate (Ronald Holshausen, Thu Oct 21 18:10:27 2021 +1100)
* 0c72c80e - chore: fixes after merging from master (Ronald Holshausen, Wed Oct 20 14:46:54 2021 +1100)
* ec265d83 - Merge branch 'master' into feat/plugins (Ronald Holshausen, Wed Oct 20 14:40:37 2021 +1100)
* 2ac17234 - chore: deprecate the old verifier functions (Ronald Holshausen, Tue Oct 19 17:42:56 2021 +1100)
* e98a91fe - chore: update to latest verifier lib (Ronald Holshausen, Tue Oct 19 17:42:07 2021 +1100)
* a3d321cb - chore: update to latest mock server crate (Ronald Holshausen, Tue Oct 19 17:28:24 2021 +1100)
* 46a404c0 - chore: update to latest pact matching crate (Ronald Holshausen, Tue Oct 19 17:20:27 2021 +1100)
* 918e5beb - fix: update to latest models and plugin driver crates (Ronald Holshausen, Tue Oct 19 17:09:48 2021 +1100)
* 7e209367 - chore: update to latest verification crate (Ronald Holshausen, Tue Oct 19 11:50:57 2021 +1100)
* 3819522d - chore: update to the latest matching and mock server crates (Ronald Holshausen, Tue Oct 19 11:34:18 2021 +1100)
* bfa04370 - fix: display the error message when the verification can not be run due to an error (Ronald Holshausen, Tue Oct 19 11:09:21 2021 +1100)
* df386c8a - chore: use the published version of pact-plugin-driver (Ronald Holshausen, Mon Oct 18 13:41:36 2021 +1100)
* 1dc6f543 - chore: bump pact_mock_server version (Ronald Holshausen, Tue Oct 12 16:36:51 2021 +1100)
* 9bbbb52e - chore: bump pact matching crate version (Ronald Holshausen, Tue Oct 12 16:24:01 2021 +1100)
* 35ff0993 - feat: record the version of the lib that created the pact in the metadata (Ronald Holshausen, Tue Oct 12 14:52:43 2021 +1100)
* 1eb37c13 - chore: use the published version of the models crate (Ronald Holshausen, Thu Oct 7 10:49:11 2021 +1100)
* 2e86c48d - Merge pull request #154 from pact-foundation/feat/xml-matchers (Matt Fellows, Tue Oct 5 16:39:18 2021 +1100)
* 9a2049c2 - feat: support XML bodies in FFI interface (Matt Fellows, Thu Sep 30 22:08:01 2021 +1000)
* d171edfd - feat: support provider branches (Matt Fellows, Wed Sep 29 22:47:21 2021 +1000)
* 6d23796f - feat(plugins): support each key and each value matchers (Ronald Holshausen, Wed Sep 29 11:10:46 2021 +1000)
* 6f20282d - Merge branch 'master' into feat/plugins (Ronald Holshausen, Tue Sep 28 14:51:34 2021 +1000)
* a8f900ab - bump version to 0.0.4 (Ronald Holshausen, Tue Sep 28 14:09:57 2021 +1000)
* 7a3c7693 - Merge branch 'master' into feat/plugins (Ronald Holshausen, Mon Sep 20 13:44:53 2021 +1000)
* b71dcabf - refactor(plugins): rename ContentTypeOverride -> ContentTypeHint (Ronald Holshausen, Tue Sep 14 15:08:52 2021 +1000)
* f55440c6 - chore: Bump verifier lib version to 0.11.0-beta.0 (Ronald Holshausen, Mon Sep 13 12:04:19 2021 +1000)
* 03ebe632 - Merge branch 'master' into feat/plugins (Ronald Holshausen, Mon Sep 13 12:01:13 2021 +1000)
* fd6f8f40 - chore: Bump pact_mock_server version to 0.8.0-beta.0 (Ronald Holshausen, Mon Sep 13 11:46:11 2021 +1000)
* 716809f6 - chore: Get CI build passing (Ronald Holshausen, Fri Sep 10 14:55:46 2021 +1000)
* ceb1c35f - Merge branch 'master' into feat/plugins (Ronald Holshausen, Tue Sep 7 10:07:45 2021 +1000)
* e8ae81b3 - refactor: matching req/res with plugins requires data from the pact and interaction (Ronald Holshausen, Thu Sep 2 11:57:50 2021 +1000)
* b9aa7ecb - feat(Plugins): allow plugins to override text/binary format of the interaction content (Ronald Holshausen, Mon Aug 30 10:48:04 2021 +1000)
* 0c5cede2 - chore: bump models crate to 0.2 (Ronald Holshausen, Mon Aug 23 12:56:14 2021 +1000)
* 248629e1 - chore: fix build after merge from master (Ronald Holshausen, Mon Aug 23 10:42:44 2021 +1000)
* 75e13fd8 - Merge branch 'master' into feat/plugins (Ronald Holshausen, Mon Aug 23 10:33:45 2021 +1000)
* b75fea5d - Merge branch 'master' into feat/plugins (Ronald Holshausen, Wed Aug 18 12:27:41 2021 +1000)
* 2662241e - feat(plugins): Call out to plugins when comparing content owned by the plugin during verification (Ronald Holshausen, Fri Aug 13 14:29:30 2021 +1000)
* bdfc6f02 - feat(plugins): Load required plugins when verifying a V4 pact (Ronald Holshausen, Wed Aug 11 14:23:54 2021 +1000)
* dfe3cd42 - chore: bump minor version of Pact verifier libs (Ronald Holshausen, Mon Aug 9 15:10:47 2021 +1000)

# 0.0.3 - support native TLS certs + updated verifier FFI functions

* 42be9eb8 - feat: add FFI functions to extract logs from a verifcation run (Ronald Holshausen, Tue Sep 28 12:48:40 2021 +1000)
* 40cf1ab9 - chore: mark pactffi_logger_attach_sink as unsafe #148 (Ronald Holshausen, Fri Sep 24 11:36:38 2021 +1000)
* ab89152e - Merge pull request #150 from tienvx/make-state-change-url-optional (Ronald Holshausen, Tue Sep 21 09:20:54 2021 +1000)
* df715cd5 - feat: support native TLS. Fixes #144 (Matt Fellows, Mon Sep 20 13:00:33 2021 +1000)
* 339a9504 - feat: make state change url optional (tienvx, Mon Sep 20 12:13:29 2021 +0700)
* dab70272 - feat: add verifier ffi function set consumer filters (tienvx, Tue Sep 14 23:47:14 2021 +0700)
* 36f7e477 - fix: fix missing last tag (tienvx, Tue Sep 14 23:51:02 2021 +0700)
* 4e02722e - Handle required and optional parameters (tienvx, Fri Sep 10 21:56:45 2021 +0700)
* ad73c9af - Extract function get_tags to reuse code (tienvx, Fri Sep 10 21:55:36 2021 +0700)
* 05f4c3de - feat: add verifier ffi function set verification options (tienvx, Wed Sep 8 23:48:13 2021 +0700)
* 971b980e - chore: fix clippy warnings (Ronald Holshausen, Fri Sep 10 17:31:16 2021 +1000)
* 0bb96329 - chore: fix clippy warnings (Ronald Holshausen, Fri Sep 10 17:15:17 2021 +1000)
* b8e51313 - Merge pull request #137 from tienvx/ffi-function-update-provider-state (Ronald Holshausen, Sat Sep 4 13:04:50 2021 +1000)
* 47e940a8 - test(ffi verifier): remove unused import (Mike Geeves, Tue Aug 31 10:11:37 2021 +0100)
* d5167056 - feat(ffi verifier cli): simplify duplicated conversion for default_value, env, possible_values (Mike Geeves, Mon Aug 30 21:29:34 2021 +0100)
* fd9ea9c3 - feat(ffi verifier cli): attributes long/short/help can be simplified (Mike Geeves, Mon Aug 30 21:06:23 2021 +0100)
* 9e582360 - chore: add verifier ffi function update provider state (tienvx, Sun Aug 29 22:20:28 2021 +0700)
* 55985d0a - feat(ffi verifier cli): add in support for ENVs (Mike Geeves, Fri Aug 27 15:59:56 2021 +0100)
* 4a5cdb82 - Merge branch 'master' into feat/ffi_arguments (Mike Geeves, Fri Aug 27 09:57:52 2021 +0100)
* 84957fb9 - feat(ffi verifier cli): verify we can deserialize the json from cli_args, and there are some args (Mike Geeves, Fri Aug 27 09:55:24 2021 +0100)
* 906661cb - feat(ffi verifier cli): split out flags and options (Mike Geeves, Thu Aug 26 11:45:18 2021 +0100)
* 491c23fb - feat(ffi verifier): add multiple to CLI JSON output (Mike Geeves, Wed Aug 25 15:58:00 2021 +0100)
* 46135a16 - chore: add verifier FFI functions for directory, URL and Pact broker sources (Ronald Holshausen, Tue Aug 24 10:14:46 2021 +1000)
* bbae32da - feat(ffi verify): add in default values, start looking at flags (Mike Geeves, Tue Aug 24 00:25:56 2021 +0100)
* ffcabb63 - feat(ffi verifier): add possible_values (Mike Geeves, Mon Aug 23 10:21:16 2021 +0100)
* 5a32f04d - feat(ffi verifier): bump serde version to latest (Mike Geeves, Mon Aug 23 09:55:31 2021 +0100)
* f64b0ead - feat(ffi verifier): revert unwanted changes (Mike Geeves, Mon Aug 23 09:53:31 2021 +0100)
* e8247e55 - feat(ffi verifier): merge master, fix conflicts (Mike Geeves, Mon Aug 23 09:51:24 2021 +0100)
* e557ce27 - feat(ffi verifier): move pactffi_verifier_cli_args to mod.rs, tidy, add docs (Mike Geeves, Mon Aug 23 09:45:54 2021 +0100)
* 4982bfc7 - chore: update FFI readme (Ronald Holshausen, Mon Aug 23 10:31:08 2021 +1000)
* f8d98dcb - feat(ffi verifier): added a crude method to pull out CLI arguments, and make available via FFI (Mike Geeves, Sun Aug 22 19:45:42 2021 +0100)
* 50fcd409 - chore: re-enable cmake build for pact-ffi (Ronald Holshausen, Sun Aug 22 16:20:25 2021 +1000)
* eaefe4d2 - chore: correct the conan recipes (Ronald Holshausen, Sun Aug 22 16:18:50 2021 +1000)
* 72125560 - bump version to 0.0.3 (Ronald Holshausen, Sun Aug 22 15:51:25 2021 +1000)

# 0.0.2 - Bugfix Release

* 9370327c - feat(FFI): Added initial verifier FFI prototype (Ronald Holshausen, Sun Aug 22 15:01:17 2021 +1000)
* c274ca1a - fix: use the pacts for verification endpoint if the conusmer selectors are specified #133 (Ronald Holshausen, Sun Aug 22 11:51:22 2021 +1000)
* 3215821e - chore: correct the OSX release (Ronald Holshausen, Tue Aug 17 12:47:20 2021 +1000)
* 64cf38e9 - bump version to 0.0.2 (Ronald Holshausen, Tue Aug 17 11:00:54 2021 +1000)

# 0.0.1 - M1 architecture support + Bugfixes

* a9940325 - chore: release m1 arm package for new Mac hardware (Matt Fellows, Wed Aug 11 22:57:46 2021 +1000)
* b5a7b779 - feat: support new selectors (Matt Fellows, Mon Aug 9 13:27:33 2021 +1000)
* 8bcd1c7e - fix: min/max type matchers must not apply the limits when cascading (Ronald Holshausen, Sun Aug 8 15:50:40 2021 +1000)
* 738e9961 - chore: generate both a C and C++ header (Ronald Holshausen, Sun Aug 8 14:28:11 2021 +1000)
* 6124ed0b - refactor: Introduce DocPath struct for path expressions (Caleb Stepanian, Thu Jul 29 12:27:32 2021 -0400)
* 9baa714d - chore: bump minor version of matching crate (Ronald Holshausen, Fri Jul 23 14:03:20 2021 +1000)
* 533c9e1f - chore: bump minor version of the Pact models crate (Ronald Holshausen, Fri Jul 23 13:15:32 2021 +1000)
* 3dccf866 - refacfor: moved the pact structs to the models crate (Ronald Holshausen, Sun Jul 18 16:58:14 2021 +1000)
* 372cbdd1 - chore: remove CMake step from CI build (Ronald Holshausen, Sun Jul 18 15:57:30 2021 +1000)
* 996761a6 - chore: remove CMake step from CI build (Ronald Holshausen, Sun Jul 18 15:26:31 2021 +1000)
* 685a2df2 - chore: cmake is failing on CI to find cargo (Ronald Holshausen, Sun Jul 18 15:12:51 2021 +1000)
* e8046d84 - refactor: moved interaction structs to the models crate (Ronald Holshausen, Sun Jul 18 14:36:03 2021 +1000)
* 0c528ea0 - chore: update pact_ffi readme (Ronald Holshausen, Mon Jul 12 15:28:53 2021 +1000)
* 95c873ec - chore: correct conan packages (Ronald Holshausen, Mon Jul 12 11:18:44 2021 +1000)
* bd989b8f - chore: add homepage and repositry URL to FFI manifest (Ronald Holshausen, Mon Jul 12 10:25:36 2021 +1000)
* 5e725866 - chore: add conan publish to release script (Ronald Holshausen, Mon Jul 12 10:23:33 2021 +1000)
* 6d1ff318 - fix: conan packages for pact_ffi (Ronald Holshausen, Mon Jul 12 09:23:48 2021 +1000)
* d7079e43 - bump version to 0.0.1 (Ronald Holshausen, Sun Jul 11 17:47:18 2021 +1000)

# 0.0.0 - Initial Release
