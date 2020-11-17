module.exports = {
  future: {
    removeDeprecatedGapUtilities: true,
    purgeLayersByDefault: true,
  },
  purge: {
    mode: "layers",
    layers: ["base", "components", "utilities"],
    content: ["../src/main.rs"],
  },
  theme: {},
  variants: {
    display: ["responsive", "hover", "focus", "group-focus", "focus-within"],
  },
  plugins: [],
};
