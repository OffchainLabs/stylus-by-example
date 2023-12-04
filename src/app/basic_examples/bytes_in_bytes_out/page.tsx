import { css } from "@/styled-system/css";
import { CodePanel } from "@/components/app/code_panel";

import { main, cargo } from "@/data/examples/basic_examples/bytes_in_bytes_out";

export default function Page() {
  return (
    <>
      <h2
        className={css({
          fontSize: "2xl",
          fontWeight: "bold",
          mt: "2",
          mb: "4",
        })}
      >
        Bytes In, Bytes Out
      </h2>
      <p></p>
      <h3
        className={css({
          fontFamily: "mono",
          mt: "4",
          mb: "2",
          fontSize: "16px",
        })}
      >
        src/main.rs
      </h3>
      <CodePanel code={main} language="rust" />
      <h3
        className={css({
          fontFamily: "mono",
          mt: "4",
          mb: "2",
          fontSize: "16px",
        })}
      >
        Cargo.toml
      </h3>
      <CodePanel code={cargo} language="toml" />
    </>
  );
}
