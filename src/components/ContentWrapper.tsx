import { css } from "@emotion/react";

export const ContentsWrapper: React.FC<{ children }> = ({ children }) => (
  <div
    css={css`
      overflow: hidden;
      flex-direction: column;
      flex: 1 1 0%;
      display: flex;
    `}
  >
    <main className="content">
      <section
        css={css`
          flex-direction: column;
          flex: 1 1 0%;
          display: flex;
        `}
      >
        <div
          css={css`
            flex-grow: 2;
            flex-basis: 0%;
          `}
          style={{ flexGrow: 2, flexBasis: "0%" }}
        >
          {children}
        </div>
      </section>
    </main>
  </div>
);
