import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { dracula, prism } from "react-syntax-highlighter/dist/esm/styles/prism";
import { css } from "@/styled-system/css";
import { CopyButton } from "@/components/app/copy_button";

interface CodePanelProps extends React.HtmlHTMLAttributes<HTMLDivElement> {
  children: string;
  language?: string;
}

export function CodePanel({ children, language = "rust" }: CodePanelProps) {
  return (
    <>
      <div
        className={css({
          borderColor: "stone.900",
          borderWidth: "1px",
          borderRadius: "lg",
          overflow: "hidden",
          maxW: "4xl",
          display: { _light: "none" },
          position: "relative",
        })}
      >
        <CopyButton content={children} />
        <SyntaxHighlighter
          language={language}
          style={dracula}
          customStyle={{ margin: 0 }}
          codeTagProps={{
            style: { fontFamily: "var(--font-mono)", fontSize: "16px" },
          }}
          wrapLines={true}
        >
          {children}
        </SyntaxHighlighter>
      </div>
      <div
        className={css({
          borderColor: { _light: "stone.200", _dark: "stone.700" },
          borderWidth: "1px",
          borderRadius: "lg",
          overflow: "hidden",
          display: { _dark: "none" },
          position: "relative",
        })}
      >
        <CopyButton content={children} />
        <SyntaxHighlighter
          language={language}
          style={prism}
          customStyle={{ margin: 0 }}
          codeTagProps={{
            style: { fontFamily: "var(--font-mono)", fontSize: "16px" },
          }}
          wrapLines={true}
        >
          {children}
        </SyntaxHighlighter>
      </div>
    </>
  );
}
