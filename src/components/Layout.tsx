import { ContentsWrapper } from "./ContentWrapper";
import { BodyWrapper } from "./BodyWrapper";
import { TopNavbar } from "./TopNavbar";

export const Layout: React.FC<{ children }> = ({ children }) => {
  return (
    <BodyWrapper>
      <TopNavbar />
      <ContentsWrapper>{children}</ContentsWrapper>
    </BodyWrapper>
  );
};
