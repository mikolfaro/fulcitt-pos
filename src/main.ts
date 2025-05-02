import "./style.css"

import { createApp } from "vue";
import { createMemoryHistory, createRouter } from "vue-router";
import App from "./App.vue";
import PosView from "./components/views/PosView.vue";
import SettingsView from "./components/views/SettingsView.vue";
import ProductSettingsView from "./components/views/settings/ProductsView.vue";

const routes = [
  {
    path: '/',
    component: PosView
  },
  {
    path: '/settings',
    component: SettingsView,
    children: [
      {
        path: 'products',
        component: ProductSettingsView,
      }
    ]
  }
];

const router = createRouter({ history: createMemoryHistory(), routes })

createApp(App)
  .use(router)
  .mount("#app");
