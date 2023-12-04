import SyntaxHighlighter from "react-syntax-highlighter";
import { gruvboxDark } from "react-syntax-highlighter/dist/esm/styles/hljs";
import { css } from "@/styled-system/css";

interface CodePanelProps extends React.HtmlHTMLAttributes<HTMLDivElement> {
  code: string;
}

export function CodePanel({ code }: CodePanelProps) {
  return (
    <div
      className={css({
        borderColor: "stone.900",
        borderWidth: "1px",
        borderRadius: "lg",
        overflow: "hidden",
      })}
    >
      <SyntaxHighlighter
        language="rust"
        style={gruvboxDark}
        codeTagProps={{ style: { fontFamily: "var(--font-mono)" } }}
        wrapLines={true}
      >
        {code}
      </SyntaxHighlighter>
    </div>
  );
}
