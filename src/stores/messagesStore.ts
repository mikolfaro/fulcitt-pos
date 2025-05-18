import { defineStore } from "pinia";
import { AppMessage } from "../lib";

export const useMessagesStore = defineStore('messages', {
  state: () => ({
    messages: [] as AppMessage[]
  }),
  actions: {
    addMessage(error: AppMessage) {
      this.messages.push(error)
    },
    addInvalidInput(message: string) {
      this.messages.push({ type: 'InvalidInput', message })
    },
    addSuccess(message: string) {
      this.messages.push({ type: 'Success', message })
    },
    addUnknownError(message: string) {
      this.messages.push({ type: 'Unknown', message })
    },
    removeMessage(index: number) {
      this.messages.splice(index, 1)
    },
    clearMessages() {
      this.messages = []
    }
  }
})
