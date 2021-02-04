/** @type {import("snowpack").SnowpackUserConfig } */
// Fire up a proxy from `/api*` to the api server running on localhost
const httpProxy = require('http-proxy')
const proxy = httpProxy.createServer({ target: 'http://0.0.0.0' })

module.exports = {
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
    {
      src: '/api/.*',
      dest: (req, res) => proxy.web(req, res),
    },
    { match: 'routes', src: '.*', dest: '/index.html' },
  ],
  optimize: {
    /* Example: Bundle your final build: */
    bundle: true,
    minify: true,
    target: 'es2017',
  },
  packageOptions: {
    /* ... */
  },
  devOptions: {
    /* ... */
  },
  buildOptions: {
    /* ... */
  },
}
