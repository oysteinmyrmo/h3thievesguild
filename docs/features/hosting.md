# Hosting

The site is hosted on GitHub Pages at <https://oysteinmyrmo.github.io/h3thievesguild/>, built and
deployed by GitHub Actions from main.

## What is done

- `.github/workflows/build.yml` deploys on every push to main that changes more than
  documentation. The `web` job, after its type check, unit tests and browser tests, builds the
  site with `npm run build` and uploads `web/dist` with `actions/upload-pages-artifact`. The
  `deploy` job waits for every test job (`build`, `lint`, `wasm` and `web`), then publishes it with
  `actions/configure-pages` and `actions/deploy-pages`. Branches and pull requests are tested
  but not deployed.
- Only the deploy job can write to Pages (`pages: write`, and `id-token: write` for the deployment
  token); the rest of the workflow can only read the repository.
- On main a newer push waits for the running workflow instead of stopping it, so a deployment is
  never stopped halfway.
- No 404 page is needed. Vite builds with `base: "./"`, so every file is found relative to the
  page, at any sub-path such as `/h3thievesguild/`; and pages are addressed by the URL hash
  (`#/saves/...`, `#/calculators`), so the server is only ever asked for `index.html` and the
  files beside it.

## One-time setup

In the repository on GitHub, under **Settings → Pages → Build and deployment**, set **Source** to
**GitHub Actions**. Until then the deploy job fails.

GitHub Pages is free for public repositories; a private one needs a paid plan, and its site is
public all the same.
