import PosView from "./components/views/PosView.vue";
import ReportView from "./components/views/ReportView.vue";
import SettingsView from "./components/views/SettingsView.vue";
import LayoutView from "./components/views/settings/LayoutView.vue";
import PrinterView from "./components/views/settings/PrinterView.vue";
import ProductSettingsView from "./components/views/settings/ProductsView.vue";
import UIView from "./components/views/settings/UIView.vue";

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
