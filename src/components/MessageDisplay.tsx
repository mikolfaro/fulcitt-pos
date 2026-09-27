import { useFluent } from "fluent-vue";
import { defineComponent } from "vue";
import { useMessagesStore } from "../stores/messagesStore";

export default defineComponent({
  name: "MessageDisplay",
  setup() {
    const { $t } = useFluent();
    const messages = useMessagesStore();

    function getAlertType(type: string) {
      switch (type) {
        case "InvalidInput":
          return "alert-warning";
        case "Database":
        case "Printer":
        case "Internal":
        case "MutexPoisoned":
          return "alert-error";
        case "Success":
          return "alert-success";
        default:
          console.warn("Unknown error", type);
          return "alert-error";
      }
    }

    function getTitle(type: string) {
      return $t(`app-message-generic-${type}`);
    }

    function dismissMessage(index: number) {
      messages.removeMessage(index);
    }

    return () => (
      <div class="fixed bottom-4 left-4 z-50 w-full max-w-md space-y-2">
        {Object.entries(messages.messages).map(([index, message]) => (
          <div
            key={index}
            class={[
              "alert alert-soft shadow-lg flex",
              getAlertType(message.type),
            ]}
            role="alert"
          >
            <div class="grow">
              {message.type !== "Success" && (
                <h3 class="font-bold">{getTitle(message.type)}</h3>
              )}
              <div class="text-xs">{message.message}</div>
            </div>
            <div>
              <button
                class="btn btn-sm btn-circle"
                onClick={() => dismissMessage(Number(index))}
              >
                ✕
              </button>
            </div>
          </div>
        ))}
      </div>
    );
  },
});
