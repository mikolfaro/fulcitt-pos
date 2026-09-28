import PosView from "./components/views/PosView";
import ReportView from "./components/views/ReportView";
import SettingsView from "./components/views/SettingsView";
import LayoutView from "./components/views/settings/LayoutView";
import PrinterView from "./components/views/settings/PrinterView";
import ProductSettingsView from "./components/views/settings/ProductsView";
import UIView from "./components/views/settings/UIView";

export default [
  {
    path: "/",
    component: PosView,
  },
  {
    path: "/report",
    component: ReportView,
  },
  {
    path: "/settings",
    component: SettingsView,
    children: [
      {
        path: "",
        component: ProductSettingsView,
      },
      {
        path: "layout",
        component: LayoutView,
      },
      {
        path: "printer",
        component: PrinterView,
      },
      {
        path: "ui",
        component: UIView,
      },
    ],
  },
];
