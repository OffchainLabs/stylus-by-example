import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import {
  coldarkCold,
  dracula,
  oneDark,
  oneLight,
  prism,
} from "react-syntax-highlighter/dist/esm/styles/prism";
import { css } from "@/styled-system/css";

interface CodePanelProps extends React.HtmlHTMLAttributes<HTMLDivElement> {
  code: string;
}

export function CodePanel({ code }: CodePanelProps) {
  return (
    <>
      <div
        className={css({
          borderColor: "stone.900",
          borderWidth: "1px",
          borderRadius: "lg",
          overflow: "hidden",
          display: { _light: "none" },
        })}
      >
        <SyntaxHighlighter
          language="rust"
          style={oneDark}
          customStyle={{ margin: 0 }}
          codeTagProps={{
            style: { fontFamily: "var(--font-mono)", fontSize: "16px" },
          }}
          wrapLines={true}
        >
          {code}
        </SyntaxHighlighter>
      </div>
      <div
        className={css({
          borderColor: "stone.900",
          borderWidth: "1px",
          borderRadius: "lg",
          overflow: "hidden",
          display: { _dark: "none" },
        })}
      >
        <SyntaxHighlighter
          language="rust"
          style={prism}
          customStyle={{ margin: 0 }}
          codeTagProps={{
            style: { fontFamily: "var(--font-mono)", fontSize: "16px" },
          }}
          wrapLines={true}
        >
          {code}
        </SyntaxHighlighter>
      </div>
    </>
  );
}
