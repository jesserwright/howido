/** @type {import("snowpack").SnowpackUserConfig } */
// Fire up a proxy from `/api*` to the api server running on localhost

require('dotenv').config({ path: '../.env' })

const { HOST, PORT } = process.env

module.exports = {
  env: {
    API_URL: `http://${HOST}:${PORT}`
  },
  mount: {
    public: { url: '/', static: true },
    src: { url: '/dist' },
  },
  plugins: [
    '@snowpack/plugin-react-refresh',
    '@snowpack/plugin-typescript',
    '@snowpack/plugin-postcss',
  ],
  routes: [
    /* Enable an SPA Fallback in development: */
    { match: 'routes', src: '.*', dest: '/index.html' },
  ],
  optimize: {
    /* Example: Bundle your final build: */
    bundle: false,
    minify: false,
    target: 'es2020',
  },
  packageOptions: {
    /* ... */
  },
  devOptions: {
    open: 'none',
    /* ... */
  },
  buildOptions: {
    /* ... */
  },
}
