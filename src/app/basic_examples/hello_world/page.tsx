import { css } from "@/styled-system/css";
import { CodePanel } from "@/components/app/code_panel";

import { hello_world } from "@/data/examples/basic_examples/hello_world";

/**
 *  TODO:
 *    - Add breadcrumbs
 *    - Add debug println example (maybe that should be hello world?)
 *    - Add a bytes in, bytes out example
 */

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
        Hello World
      </h2>
      <CodePanel>{hello_world}</CodePanel>
    </>
  );
}
