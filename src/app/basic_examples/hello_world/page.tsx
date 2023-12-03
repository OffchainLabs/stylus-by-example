import SyntaxHighlighter from "react-syntax-highlighter";
import { gruvboxDark } from "react-syntax-highlighter/dist/esm/styles/hljs";

import { css } from "@/styled-system/css";
import { Stack } from "@/styled-system/jsx";

const codeString = `#[stylus_wax::main]
fn user_main(cmd: Cmd, msg: Msg) -> Response {
    console!("user_main() executing...");
    console!("cmd: {:?} --- msg: {:?}", cmd, msg);
    match cmd {
        TRANSFER_OWNERSHIP => run(transfer_ownership, msg),
        OWNER => run(owner, msg),
        _ => Ok(vec![]),
    }
}`;

export default function Home() {
  return (
    <Stack>
      <h2
        className={css({
          fontSize: "2xl",
          fontWeight: "bold",
        })}
      >
        Hello World
      </h2>
      <div
        className={css({
          borderColor: "stone.950",
          borderWidth: "1px",
          borderRadius: "lg",
          overflow: "hidden",
        })}
      >
        <SyntaxHighlighter
          language="rust"
          style={gruvboxDark}
          codeTagProps={{ style: { fontFamily: "var(--font-mono)" } }}
          showLineNumbers={true}
        >
          {codeString}
        </SyntaxHighlighter>
      </div>
    </Stack>
  );
}
