import { type Plugin, createFilter } from "vite";

import SWC from "@swc/core";

export default function (): Plugin {
  const filter = createFilter(/\.[jt]sx$/);

  return {
    name: "signal-jsx",
    enforce: "pre",

    async transform(source, id) {
      if (filter(id)) {
        return SWC.transformSync(source, {
          isModule: true,
          jsc: {
            target: "es2022",
            parser: {
              syntax: "typescript",
              tsx: true,
              decorators: true,
              dynamicImport: true,
            },
            experimental: {
              plugins: [["@westhide/swc-plugin-signal-jsx", {}]],
            },
          },
        });
      } else {
        return null;
      }
    },
  };
}
