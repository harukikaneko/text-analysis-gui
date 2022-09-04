import { css } from "@emotion/react";
import type { AppProps } from "next/app";

import "../App.css";
import { Layout } from "../components/Layout";

// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
  return (
    <Layout>
      <Component {...pageProps} />
    </Layout>
  );
}
