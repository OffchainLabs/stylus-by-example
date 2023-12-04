import { css } from "@/styled-system/css";
import { CodePanel } from "@/components/app/code_panel";

import { hello_world } from "@/data/examples/basic_examples/hello_world";

export default function Home() {
  return (
    <>
      <h2
        className={css({
          fontSize: "2xl",
          fontWeight: "bold",
        })}
      >
        Hello World
      </h2>
      <CodePanel code={hello_world} />
    </>
  );
}
