import { defineStore } from "pinia";
import { AppMessage } from "../lib";

type Messages = {[key: number]: AppMessage}
type UnknownMessage = string | {message ?: string, type ?: string }

export const useMessagesStore = defineStore('messages', {
  state: () => ({
    messages: {} as Messages,
    sequence: 0,
  }),
  actions: {
    addMessage(error: AppMessage, dismissAfter?: number) {
      const sequence = this.sequence

      this.messages[this.sequence] = error
      this.sequence += 1

      if (dismissAfter) {
        setTimeout(() => {
          this.removeMessage(sequence)
        }, dismissAfter * 1_000)
      }
    },
    addInvalidInput(message: string, dismissAfter?: number) {
      this.addMessage({ type: 'InvalidInput', message }, dismissAfter)
    },
    addSuccess(message: string) {
      this.addMessage({ type: 'Success', message }, 5)
    },
    addUnknownError(message: UnknownMessage) {
      let parsedMessage;

      if (typeof message === "string") {
        parsedMessage = { type: 'Unknown', message }
      } else if (typeof message === "object") {
        if (message?.message && message?.type) {
          parsedMessage = message as AppMessage
        } else if (message?.type) {
          parsedMessage = { type: message.type, message: '' }
        }
      }

      if (!parsedMessage) {
        console.error("Unknown message", message)
        parsedMessage = { type: 'Unknown', message: 'Unknown message' }
      }

      this.addMessage(parsedMessage)
    },
    removeMessage(index: number) {
      delete this.messages[index]
    },
    clearMessages() {
      this.messages = {}
    }
  }
})
