/** @type {import("snowpack").SnowpackUserConfig } */
// Fire up a proxy from `/api*` to the api server running on localhost
const httpProxy = require('http-proxy')

require('dotenv').config({ path: '../.env' })

const {HOST, PORT} = process.env

process.env.SNOWPACK_PUBLIC_API_URL = `http://${HOST}:${PORT}`

const proxy = httpProxy.createServer({
  target: `http://${HOST}:${PORT}`,
})

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
    // Proxy
    {
      src: '/api/.*',
      dest: (req, res) => proxy.web(req, res),
    },
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
