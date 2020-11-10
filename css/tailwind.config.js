module.exports = {
  purge: {
    mode: "layers",
    layers: ["base", "components", "utilities"],
    content: ["../templates/*.html"],
  },
  theme: {},
  variants: {},
  plugins: [],
};
