import "./style.css"

import { createApp } from "vue";
import { createMemoryHistory, createRouter } from "vue-router";
import App from "./App.vue";
import PosView from "./components/views/PosView.vue";
import CheckoutView from "./components/views/pos/CheckoutView.vue";
import PaymentView from "./components/views/pos/PaymentView.vue";
import ReportView from "./components/views/ReportView.vue";
import SettingsView from "./components/views/SettingsView.vue";
import PrintView from "./components/views/settings/PrintView.vue";
import ProductSettingsView from "./components/views/settings/ProductsView.vue";

const routes = [
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
        path: 'products',
        component: ProductSettingsView,
      },
      {
        path: 'print',
        component: PrintView,
      }
    ]
  }
];

const router = createRouter({ history: createMemoryHistory(), routes })

createApp(App)
  .use(router)
  .mount("#app")
