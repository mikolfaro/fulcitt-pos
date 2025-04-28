import "./style.css"

import { createApp } from "vue";
import { createMemoryHistory, createRouter, RouteRecord } from "vue-router";
import App from "./App.vue";
import PosView from "./components/PosView.vue";
import SettingsView from "./components/SettingsView.vue";
import ProductSettingsView from "./components/ProductSettingsView.vue";

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
