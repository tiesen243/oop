import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["src"],
  format: ["esm"],
  dts: true,
  minify: true,
});
