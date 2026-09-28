import { useFluent } from "fluent-vue";
import {
  cloneVNode,
  defineComponent,
  KeepAlive,
  Transition,
  type VNode,
} from "vue";
import {
  type RouteLocationNormalizedLoaded,
  RouterLink,
  RouterView,
} from "vue-router";

export default defineComponent({
  name: "SettingsView",
  setup() {
    const { $t } = useFluent();

    const subRoutes = [
      { name: $t("settings-products-menu-title"), to: "/settings" },
      { name: $t("settings-layout-menu-title"), to: "/settings/layout" },
      { name: $t("settings-printer-menu-title"), to: "/settings/printer" },
      { name: $t("settings-ui-menu-title"), to: "/settings/ui" },
    ];

    return () => (
      <div class="p-4 md:p-6">
        <h1 class="text-2xl font-bold mb-4">{$t("settings-title")}</h1>

        <div role="tablist" class="tabs tabs-lifted -mb-px-1">
          {subRoutes.map((r) => (
            <RouterLink
              role="tab"
              class="tab [--tab-border-color:oklch(var(--bc)/0.2)]"
              activeClass="tab-active"
              to={r.to}
              key={r.name}
            >
              {r.name}
            </RouterLink>
          ))}
          <a
            role="tab"
            class="tab [--tab-border-color:oklch(var(--bc)/0.2)] grow border-b!"
          ></a>
        </div>

        <div class="bg-base-100 border border-base-300 border-t-0 rounded-b-box p-6 min-h-100">
          <RouterView>
            {{
              default: ({
                Component,
                route,
              }: {
                Component: VNode | undefined;
                route: RouteLocationNormalizedLoaded;
              }) => (
                <Transition name="fade" mode="out-in">
                  <KeepAlive>
                    {Component &&
                      cloneVNode(Component, { key: route.name }, true)}
                  </KeepAlive>
                </Transition>
              ),
            }}
          </RouterView>
        </div>
      </div>
    );
  },
});
