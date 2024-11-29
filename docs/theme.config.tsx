import React from "react";
import { DocsThemeConfig } from "nextra-theme-docs";

const config: DocsThemeConfig = {
  head: (
    <>
      <title>Quick Guide Rust Programming</title>
    </>
  ),
  logo: <span>Quick Guide Rust Programming</span>,
  project: {
    link: "https://github.com/gurugio/quick-guide-rust-programming",
  },
  docsRepositoryBase: "https://github.com/gurugio/quick-guide-rust-programming",
  footer: {
    text: (
      <span>
        MIT {new Date().getFullYear()} ©{" "}
        <a href="https://make.kaist.ac.kr/" target="_blank">
          gurugio
        </a>
        .
      </span>
    ),
  },
};

export default config;
