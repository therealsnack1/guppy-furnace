import { Head, Html, Main, NextScript } from "next/document";

export default function Document() {
  return (
    <Html>
      <Head>
        <link rel="shortcut icon" href="/icons/ash-icon.svg" />
        <meta charSet="utf-8" />
        <meta name="terra-wallet" />
        <meta name="theme-color" content="#000000" />
        <meta name="description" content="White Whale Furnace" />
      </Head>
      <title>The Furnace</title>
      <body>
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
