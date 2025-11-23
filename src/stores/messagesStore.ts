import { defineStore } from "pinia";
import { AppMessage } from "../lib";

type Messages = { [key: number]: AppMessage };

function toErrorMessage(message: unknown): AppMessage {
  if (message === null) {
    return { type: "UnknownError", message: "Errore sconosciuto" };
  }

  if (typeof message === "string") {
    return { type: "GenericError", message };
  }

  if (typeof message !== "object") {
    return {
      type: "UnknownError",
      message: JSON.stringify(message),
    };
  }

  const type: string =
    "type" in message && message.type === "string"
      ? message.type
      : "UnknownError";
  const messageStr: string =
    "message" in message && message.message === "string"
      ? message.message
      : JSON.stringify(message);

  return { type, message: messageStr };
}

export const useMessagesStore = defineStore("messages", {
  state: () => ({
    messages: {} as Messages,
    sequence: 0,
  }),
  actions: {
    addMessage(error: AppMessage, dismissAfter?: number) {
      const sequence = this.sequence;

      this.messages[this.sequence] = error;
      this.sequence += 1;

      if (dismissAfter) {
        setTimeout(() => {
          this.removeMessage(sequence);
        }, dismissAfter * 1_000);
      }
    },
    addInvalidInput(message: string, dismissAfter?: number) {
      this.addMessage({ type: "InvalidInput", message }, dismissAfter);
    },
    addSuccess(message: string) {
      this.addMessage({ type: "Success", message }, 5);
    },
    addUnknownError(message: unknown) {
      const parsedMessage = toErrorMessage(message);
      this.addMessage(parsedMessage);
    },
    removeMessage(index: number) {
      delete this.messages[index];
    },
    clearMessages() {
      this.messages = {};
    },
  },
});
