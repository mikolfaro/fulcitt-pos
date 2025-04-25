import "./style.css"

import { createApp } from "vue";
import { createMemoryHistory, createRouter } from "vue-router";
import App from "./App.vue";
import PosView from "./components/PosView.vue";
import SettingsView from "./components/SettingsView.vue";

const routes = [
  {
    path: '/', component: PosView
  },
  {
    path: '/settings', component: SettingsView
  }
];

const router = createRouter({ history: createMemoryHistory(), routes })

createApp(App)
  .use(router)
  .mount("#app");
