let plugins = [
  // Process CSS imports
  require("postcss-import"),
  // Build tailwindcss
  require("tailwindcss"),
];

if (process.env.NODE_ENV === "production") {
  plugins.push(
    // Purge
    require("@fullhuman/postcss-purgecss")({
      content: ["./templates/*.html"],
    }),
    // Prefix
    require("autoprefixer"),
    // Minify
    require("cssnano")({ preset: "default" })
  );
}

module.exports = {
  plugins,
};
