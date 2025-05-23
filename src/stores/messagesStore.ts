import { defineStore } from "pinia";
import { AppMessage } from "../lib";

type Messages = {[key: number]: AppMessage}

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
    addInvalidInput(message: string, dismissAfter: number) {
      this.addMessage({ type: 'InvalidInput', message }, dismissAfter)
    },
    addSuccess(message: string) {
      this.addMessage({ type: 'Success', message }, 5)
    },
    addUnknownError(message: any) {
      if (typeof message === "string") {
        message = { type: 'Unknown', message }
      } else if (typeof message === "object") {
        if (message?.message && message?.type) {
          message = message as AppMessage
        } else {
          message = { type: 'Unknown', message }
        }
      } else {
        message = { type: 'Unknown', message }
      }

      this.addMessage(message)
    },
    removeMessage(index: number) {
      delete this.messages[index]
    },
    clearMessages() {
      this.messages = {}
    }
  }
})
