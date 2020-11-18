module.exports = {
  future: {
    removeDeprecatedGapUtilities: true,
    purgeLayersByDefault: true,
  },
  purge: {
    mode: "layers",
    layers: ["base", "components", "utilities"],
    content: ["./src/main.rs", "./templates/*.html"],
  },
  theme: {},
  variants: {},
  plugins: [],
};
