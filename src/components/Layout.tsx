import { useRouter } from "next/router";

import { ContentsWrapper } from "./ContentWrapper";
import { BodyWrapper } from "./BodyWrapper";
import { TopNavbar } from "./TopNavbar";

export const Layout: React.FC<{ children }> = ({ children }) => {
  const router = useRouter();
  return (
    <BodyWrapper>
      <TopNavbar />
      <ContentsWrapper>{children}</ContentsWrapper>
    </BodyWrapper>
  );
};
