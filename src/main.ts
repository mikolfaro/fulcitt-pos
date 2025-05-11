import "./style.css"

import { createApp } from "vue";
import { createMemoryHistory, createRouter } from "vue-router";
import { createPinia } from "pinia";

import App from "./App.vue";
import routes from "./routes"

const pinia = createPinia()
const router = createRouter({ history: createMemoryHistory(), routes })

createApp(App)
  .use(pinia)
  .use(router)
  .mount("#app")
