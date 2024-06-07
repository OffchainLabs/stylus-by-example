import { gettingStarted, basicExamples, applications } from "@/data/routes";
import { css } from "@/styled-system/css";
import { stack } from "@/styled-system/patterns";
import { Stack } from "@/styled-system/jsx";

import { SidebarLinks } from "@/components/app/sidebar_links";
import { ReactNode } from "react";

function SidebarHeading({ children }: { children: ReactNode }) {
  return (
    <h2
      className={css({
        fontSize: { base: "20px", md: "22px", "2xl": "22px" },
        fontWeight: "bold",
        mb: "1",
        px: "4",
        fontFamily: "heading",
      })}
    >
      {children}
    </h2>
  );
}

export function Sidebar() {
  return (
    <div
      className={stack({
        pt: "4",
        px: "1",
        pb: "24",
        gap: "0",
      })}
    >
      <SidebarHeading>Getting Started</SidebarHeading>
      <Stack gap={1} mb={4}>
        <SidebarLinks routes={gettingStarted} />
      </Stack>
      <SidebarHeading>Basic</SidebarHeading>
      <Stack gap={1} mb={4}>
        <SidebarLinks routes={basicExamples} />
      </Stack>
      <SidebarHeading>Applications</SidebarHeading>
      <Stack gap={1} mb={4}>
        <SidebarLinks routes={applications} />
      </Stack>
    </div>
  );
}
