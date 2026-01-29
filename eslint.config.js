import {
  defineConfigWithVueTs,
  vueTsConfigs,
} from "@vue/eslint-config-typescript";
import pluginVue from "eslint-plugin-vue";

export default defineConfigWithVueTs(
  pluginVue.configs["flat/essential"],
  vueTsConfigs.recommended,
);
