import { Head, Html, Main, NextScript } from "next/document";

export default function Document() {
  return (
    <Html>
      <Head>
        <link rel="shortcut icon" href="/icons/gash.png" />
        <meta charSet="utf-8" />
        <meta name="terra-wallet" />
        <meta name="theme-color" content="#000000" />
        <meta name="description" content="Guppy Furnace" />
      </Head>
      <title>The Furnace</title>
      <body>
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
