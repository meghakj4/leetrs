## [unreleased](https://github.com/shadowmkj/leetrs)

### ⚙️ Misc

- *(release)* Generate changelog and update cliff.toml - ([ca8969c](https://github.com/shadowmkj/leetrs/commit/ca8969c51776592f6952afba3f9ec7c09b4063e5))
## [v1.1.0](https://github.com/shadowmkj/leetrs/releases/tag/v1.1.0) - 2026-08-19

### 🚀 Features

- Add config toml file - ([790a767](https://github.com/shadowmkj/leetrs/commit/790a767c325ba0c97270a91bf6893f408bd1b3bc))
- Topic wise filtering ([#8](https://github.com/shadowmkj/leetrs/pull/8)) - ([3eb3435](https://github.com/shadowmkj/leetrs/commit/3eb3435d50102d9524593c1bed270e5ed71216df))
- *(config)* Auto-create default config.toml if missing - ([538132a](https://github.com/shadowmkj/leetrs/commit/538132a18cf47e756e24a7cb958d8816023508cb))
- *(docs)* Initialize Docusaurus site structure ([#18](https://github.com/shadowmkj/leetrs/pull/18)) - ([23275a1](https://github.com/shadowmkj/leetrs/commit/23275a17855fd47a9afbafec393f6036119abfa8))
- *(tui)* Add Ctrl+d and Ctrl+u keybindings in topic filter mode - ([18aafcb](https://github.com/shadowmkj/leetrs/commit/18aafcbae0b010fdadaa9220f16bb364b795a8b8))
- *(tui)* Add topic search with modal input modes - ([62bc2b6](https://github.com/shadowmkj/leetrs/commit/62bc2b6ea7829c3769d1c6a0e5a09388e27b9eb9))

### 🐛 Bug Fixes

- *(picker)* Return network error when offline instead of panicking - ([d0f249e](https://github.com/shadowmkj/leetrs/commit/d0f249e124ad090be776db48ae6fe0561161b9b1))
- *(tui)* Clear screen on enter and route pick errors to TUI alert popup - ([d2e6ee9](https://github.com/shadowmkj/leetrs/commit/d2e6ee93a8b5fcd442f4edf163591175c58861a0))
- *(docs)* Update logo image path and format homepage source - ([644609c](https://github.com/shadowmkj/leetrs/commit/644609c21a704e83db8a32475f47f7bd545e184b))
- *(ci)* Add environment setting for production in workflow - ([588db12](https://github.com/shadowmkj/leetrs/commit/588db12e708b9fb412bc44bf3d93f6adc1a395be))

### ⚡️ Performance

- *(tui)* Load topic filter list from embedded topics.txt - ([1dd179c](https://github.com/shadowmkj/leetrs/commit/1dd179c04337c7da558fb940382fe32e3318a498))

### 👷 CI

- *(ci)* Add code coverage workflow and git-cliff configuration - ([de7dad0](https://github.com/shadowmkj/leetrs/commit/de7dad094785659784fa4a8e0a9037eb252e4e93))
- *(ci)* Add cargo xtask automation and github actions workflows - ([e603a0c](https://github.com/shadowmkj/leetrs/commit/e603a0c519ac8beb0193d367b6c2108ece6dcfe2))
- *(ci)* Add dependabot configuration for automated updates - ([572dee3](https://github.com/shadowmkj/leetrs/commit/572dee3cb45b123a852f2d61d72769fab31909a0))

### ⚙️ Misc

- Added documentation for the entire codebase - ([2d87c17](https://github.com/shadowmkj/leetrs/commit/2d87c1712c9f60893cd116a852ba836a0e7472e0))
- Dynamic editor support, let-chains, and performance cleanups - ([7f147b0](https://github.com/shadowmkj/leetrs/commit/7f147b0e46a77564f9abb7bbbd49b1f58708ebd3))
- *(models)* Split models.rs into submodules and extract language helpers - ([9d72686](https://github.com/shadowmkj/leetrs/commit/9d72686c74d3fcd2ca58bf1c931909eb4e5fa92f))
- *(cache)* Introduce CacheService and consolidate project directory lookups - ([2df014e](https://github.com/shadowmkj/leetrs/commit/2df014e3db29518c75ee3fd2854d4b22057bb807))
- *(client)* Implement LeetCodeApi trait and generic status polling - ([2f75015](https://github.com/shadowmkj/leetrs/commit/2f7501515509e0ee5b5fc3fa6b006eb27ba6d541))
- *(cli)* Extract command handlers from main to commands module - ([b3c784b](https://github.com/shadowmkj/leetrs/commit/b3c784b276a8552c92c3dfd785fde607886b4916))
- *(services)* Introduce SubmissionService and format result output - ([37afdd3](https://github.com/shadowmkj/leetrs/commit/37afdd35047cc180c9447d9ba0430619b7cb09b7))
- *(tui)* Decompose selection screen into sub-widgets and add RAII terminal guard - ([e7f1869](https://github.com/shadowmkj/leetrs/commit/e7f18696e2d7db5ee085e586f877b9c035612d2b))
- *(config)* Register new library modules and update gitignore - ([c523bd1](https://github.com/shadowmkj/leetrs/commit/c523bd1d89e051fd7373e83a4ee878215c07c2b0))
- Update README with TUI features and quick start guidelines - ([df8ff29](https://github.com/shadowmkj/leetrs/commit/df8ff296b07a02c97dda605337e90393656e1a3c))
- *(readme)* Expand documentation, add table of contents and typography refinements - ([b3d3b42](https://github.com/shadowmkj/leetrs/commit/b3d3b42b5a98c02e37ef6e05c86049df64e3805e))
- Add a warning (in development) to readme - ([6413374](https://github.com/shadowmkj/leetrs/commit/6413374a9f3f907ccbc837654bf09ab55b07e33b))
- Update documentation, add architectural decision records, and project assets - ([d9a5a79](https://github.com/shadowmkj/leetrs/commit/d9a5a795dc3ecbe109eda186d5c35635dd5b7217))
- Address clippy lints and clean up code - ([e9c34b1](https://github.com/shadowmkj/leetrs/commit/e9c34b1844fdfa0e592974bd34fb5441839f9a1a))
- Fix markdown formatting and syntax across documentation - ([d64c594](https://github.com/shadowmkj/leetrs/commit/d64c59431659c43870c4e1e59b475ba1cd310f13))
- *(readme)* Add built with ratatui badge - ([a7c875b](https://github.com/shadowmkj/leetrs/commit/a7c875b5fb10f0817d4b0cfd37bd563464cd4aa7))
- *(refactor)* Clean up and remove redundant unit tests - ([b3519b3](https://github.com/shadowmkj/leetrs/commit/b3519b3bdddf143edda6cecf9160b9417ca16150))
- *(dependabot)* Group patch updates and tui crates - ([9963a57](https://github.com/shadowmkj/leetrs/commit/9963a57f6c4e7fab79efa10a828846b84462fb90))
- *(cargo)* Format TOML configuration files - ([c0956c8](https://github.com/shadowmkj/leetrs/commit/c0956c84c765317f543c6170e28b6bb55a9b5074))
- *(release)* Bump version to 1.1.0 - ([0aea802](https://github.com/shadowmkj/leetrs/commit/0aea8028c8401ea915dc268d861f4f63afb22339))

### 👥 New Contributors

- `@shadowmkj` made their first contribution

- `@dependabot[bot]` made their first contribution in [#31](https://github.com/shadowmkj/leetrs/pull/31)

- `@deepansh11` made their first contribution in [#8](https://github.com/shadowmkj/leetrs/pull/8)

- `@SykikXO` made their first contribution in [#5](https://github.com/shadowmkj/leetrs/pull/5)

- `@spencer-hann` made their first contribution in [#4](https://github.com/shadowmkj/leetrs/pull/4)

- `@meghakj4` made their first contribution in [#3](https://github.com/shadowmkj/leetrs/pull/3)

