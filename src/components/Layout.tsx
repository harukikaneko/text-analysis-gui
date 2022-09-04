import { icons } from "./atoms/Icon";
import { useRouter } from "next/router";

import { ContentsWrapper } from "./ContentWrapper";
import { BodyWrapper } from "./BodyWrapper";
import "react-minimal-side-navigation/lib/ReactMinimalSideNavigation.css";
import { Button, Dropdown, Link, Navbar } from "@nextui-org/react";
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
