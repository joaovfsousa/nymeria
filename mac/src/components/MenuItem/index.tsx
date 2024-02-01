import "./index.css";
import { FC } from "react";

type MenuItemProps = {
  title: string;
  onClick?: () => void;
};

export const MenuItem: FC<MenuItemProps> = ({ title, onClick }) => {
  return (
    <div className="container" onClick={onClick}>
      {title}
    </div>
  );
};
