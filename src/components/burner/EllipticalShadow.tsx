import React, { ReactNode } from "react";

interface EllipticalShadowProps {
  children: ReactNode;
}

const EllipticalShadow = ({ children }: EllipticalShadowProps) => {
  return (
    <>
      <div
        style={{
          width: "1000px",
          height: "500px",
          background: "black",
          borderRadius: "50%",
          opacity: "0.5",
          filter: "blur(100px)",
          position: "absolute",
          zIndex: "0",
        }}
      ></div>
      <div
        style={{
          zIndex: "1",
        }}
      >
        {children}
      </div>
    </>
  );
};

export default EllipticalShadow;
