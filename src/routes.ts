import PosView from "./components/views/PosView.vue";
import CheckoutView from "./components/views/pos/CheckoutView.vue";
import PaymentView from "./components/views/pos/PaymentView.vue";
import ReportView from "./components/views/ReportView.vue";
import SettingsView from "./components/views/SettingsView.vue";
import PrinterView from "./components/views/settings/PrinterView.vue";
import ProductSettingsView from "./components/views/settings/ProductsView.vue";
import LayoutView from "./components/views/settings/LayoutView.vue";

export default [
  {
    path: '/',
    component: PosView,
    children: [
      { path: '', component: CheckoutView },
      {
        path: 'checkout',
        component: PaymentView
      }
    ]
  },
  {
    path: '/report',
    component: ReportView
  },
  {
    path: '/settings',
    component: SettingsView,
    children: [
      {
        path: '',
        component: ProductSettingsView,
      },
      {
        path: 'layout',
        component: LayoutView,
      },
      {
        path: 'printer',
        component: PrinterView,
      }
    ]
  }
];

