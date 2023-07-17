import React, { ReactNode } from "react";

interface BackgroundProps {
  children: ReactNode;
}

const Background = ({ children }: BackgroundProps) => {
  return (
    <div
      style={{
        backgroundColor: "black",
        minHeight: "100%",
        minWidth: "100%",
        width: "100%",
        height: "auto",
        position: "fixed",
        top: 0,
        left: 0,
      }}
    >
      <div
        style={{
          backgroundImage: `url(images/home-background.png)`,
          backgroundColor: "black",
          backgroundSize: "cover",
          backgroundPosition: "center",
          opacity: "0.8",
          minHeight: "100%",
          minWidth: "100%",
          width: "100%",
          height: "auto",
          position: "fixed",
          top: 0,
          left: 0,
        }}
      >
        {children}
      </div>
    </div>
  );
};

export default Background;
