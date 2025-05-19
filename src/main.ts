import "./style.css"

import { createApp } from "vue";
import { createI18n } from "vue-i18n";
import { createMemoryHistory, createRouter } from "vue-router";
import { createPinia } from "pinia";
import { debug, error, info, trace, warn } from '@tauri-apps/plugin-log';

import App from "./App.vue";
import routes from "./routes"
import it from "./locales/it"

function forwardConsole(
  fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',
  logger: (message: string) => Promise<void>
) {
  const original = console[fnName];
  console[fnName] = (...args) => {
    original(...args)

    const message = args.shift()
    const otherArgs = args.map(a => JSON.stringify(a)).join(' ')
    logger(`${message}, ${otherArgs}`)
  };
}

forwardConsole('log', trace);
forwardConsole('debug', debug);
forwardConsole('info', info);
forwardConsole('warn', warn);
forwardConsole('error', error);

type MessageSchema = typeof it
const i18n = createI18n<[MessageSchema], 'it'>({
  legacy: false,
  locale: 'it',
  messages: { it }
})

const pinia = createPinia()
const router = createRouter({ history: createMemoryHistory(), routes })

createApp(App)
  .use(i18n)
  .use(pinia)
  .use(router)
  .mount("#app")
