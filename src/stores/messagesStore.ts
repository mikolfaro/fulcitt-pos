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
    addInputError(message: string) {
      this.messages.push({ type: 'InputValue', message })
    },
    addSuccess(message: string) {
      this.messages.push({ type: 'Success', message })
    },
    removeMessage(index: number) {
      this.messages.splice(index, 1)
    },
    clearMessages() {
      this.messages = []
    }
  }
})
