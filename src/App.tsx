import { defineComponent, onBeforeMount } from "vue";
import { RouterView } from "vue-router";
import MessageDisplay from "./components/MessageDisplay";
import NavBar from "./components/NavBar";
import { useProductShortcutsStore } from "./stores/productShortcuts";
import { useUiSettings } from "./stores/uiSettings";

export default defineComponent({
  name: "App",
  setup() {
    const uiSettings = useUiSettings();
    const productShortcuts = useProductShortcutsStore();

    onBeforeMount(async () => {
      try {
        await uiSettings.load();
        await productShortcuts.load();
      } catch (error) {
        console.error("Error loading settings:", error);
      }
    });

    return () => (
      <div class="flex flex-col h-screen">
        <NavBar />
        <main class="grow overflow-auto">
          <RouterView />

          <MessageDisplay />
        </main>
      </div>
    );
  },
});
