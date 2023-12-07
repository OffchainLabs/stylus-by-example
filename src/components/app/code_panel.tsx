import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import {
  coldarkCold,
  dracula,
  oneDark,
  oneLight,
  prism,
} from "react-syntax-highlighter/dist/esm/styles/prism";
import { css } from "@/styled-system/css";
import { HamburgerMenuIcon, CopyIcon } from "@radix-ui/react-icons";
import { Button } from "@/components/ui/button";
interface CodePanelProps extends React.HtmlHTMLAttributes<HTMLDivElement> {
  children: string;
  language?: string;
}

function CopyButton() {
  return (
    <Button
      variant="ghost"
      className={css({
        background: "blue.400",
        opacity: "0.7",
        h: "42px",
        w: "42px",
        px: "0",
        position: "absolute",
        top: "8px",
        right: "8px",
        rounded: "sm",
        _hover: {
          opacity: "1",
          cursor: "pointer",
        },
        _active: {
          background: "orange.200",
        },
      })}
    >
      <CopyIcon
        className={css({
          h: "32px",
          w: "32px",
        })}
      />
    </Button>
  );
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
        <CopyButton />
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
        <CopyButton />
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
