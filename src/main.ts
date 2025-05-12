import "./style.css"

import { createApp } from "vue";
import { createMemoryHistory, createRouter } from "vue-router";
import { createPinia } from "pinia";
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';

import App from "./App.vue";
import routes from "./routes"

function forwardConsole(
  fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',
  logger: (message: string) => Promise<void>
) {
  const original = console[fnName];
  console[fnName] = (message) => {
    original(message);
    logger(message);
  };
}

forwardConsole('log', trace);
forwardConsole('debug', debug);
forwardConsole('info', info);
forwardConsole('warn', warn);
forwardConsole('error', error);

const pinia = createPinia()
const router = createRouter({ history: createMemoryHistory(), routes })

createApp(App)
  .use(pinia)
  .use(router)
  .mount("#app")
