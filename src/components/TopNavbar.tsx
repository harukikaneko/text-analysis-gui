import { Dropdown, Link, Navbar } from "@nextui-org/react";
import { icons } from "./atoms/Icon";

export const TopNavbar: React.FC = () => (
  <Navbar isCompact isBordered variant="floating">
    <Navbar.Content hideIn="xs" variant="underline">
      <Navbar.Link href="/">Home</Navbar.Link>
      <Dropdown isBordered>
        <Navbar.Item>
          <Dropdown.Button
            auto
            light
            css={{
              px: 0,
              dflex: "center",
              svg: { pe: "none" },
            }}
            iconRight={icons.chevronDown}
            ripple={false}
          >
            Analysis
          </Dropdown.Button>
        </Navbar.Item>
        <Dropdown.Menu
          aria-label="analysis"
          css={{
            $$dropdownMenuWidth: "340px",
            $$dropdownItemHeight: "70px",
            "& .nextui-dropdown-item": {
              py: "$4",
              // dropdown item left icon
              svg: {
                color: "$secondary",
                mr: "$4",
              },
              // dropdown item title
              "& .nextui-dropdown-item-content": {
                w: "100%",
                fontWeight: "$semibold",
              },
            },
          }}
        >
          <Dropdown.Item
            key="autoscaling"
            showFullDescription
            icon={icons.nouns}
          >
            <Link href="/analysis/nouns">Count by Noun</Link>
          </Dropdown.Item>
        </Dropdown.Menu>
      </Dropdown>
    </Navbar.Content>
  </Navbar>
);
