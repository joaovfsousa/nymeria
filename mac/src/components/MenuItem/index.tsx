import "./index.css";
import { FC } from "react";

type MenuItemProps = {
  title: string;
};

export const MenuItem: FC<MenuItemProps> = ({ title }) => {
  return <div className="container">{title}</div>;
};
