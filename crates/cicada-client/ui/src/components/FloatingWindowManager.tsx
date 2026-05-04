import React from "react";
import FloatingWindow from "./FloatingWindow";

const FloatingWindowManager: React.FC = () => {
  return (
    <div style={{ display: "none" }}>
      <FloatingWindow />
    </div>
  );
};

export default FloatingWindowManager;
