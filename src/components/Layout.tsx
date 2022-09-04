import { BiHome, BiAnalyse, BiFileFind } from "react-icons/bi";
import { css } from "@emotion/react";
import { useRouter } from "next/router";

import { ContentsWrapper } from "./ContentWrapper";
import { BodyWrapper } from "./BodyWrapper";
import { Navigation } from "react-minimal-side-navigation";
import "react-minimal-side-navigation/lib/ReactMinimalSideNavigation.css";

export const Layout: React.FC<{ children }> = ({ children }) => {
  const router = useRouter();
  return (
    <BodyWrapper>
      <div
        css={css`
          height: 100vh;
          display: flex;
        `}
        className="flex h-screen bg-gray-200"
      >
        <div
          css={css`
            background-color: white;
          `}
        >
          <Navigation
            activeItemId="/"
            onSelect={({ itemId }) => {
              router.push(itemId);
            }}
            items={[
              {
                title: "Home",
                itemId: "/",
                elemBefore: () => <BiHome name="home" />,
              },
              {
                title: "Analysis",
                itemId: "/analysis",
                elemBefore: () => <BiAnalyse name="analysis" />,
                subNav: [
                  {
                    title: "CountsByNoun",
                    itemId: "/analysis/nouns",
                    elemBefore: () => <BiFileFind name="cloud-snow" />,
                  },
                ],
              },
            ]}
          />
        </div>
        <ContentsWrapper>{children}</ContentsWrapper>
      </div>
    </BodyWrapper>
  );
};
