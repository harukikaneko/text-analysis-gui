import { css } from "@emotion/react";

export const BodyWrapper: React.FC<{ children }> = ({ children }) => (
  <div
    css={css`
      position: relative;
      min-height: 100vh;
    `}
  >
    <main
      css={css`
        width: 100%;
        min-height: 100vh;
      `}
    >
      {children}
    </main>
  </div>
);
