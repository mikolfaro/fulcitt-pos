<template>
  <div class="fixed bottom-4 left-4 z-50 w-full max-w-md space-y-2">
    <div
      v-for="(message, index) in messages.messages"
      :key="index"
      :class="['alert alert-soft shadow-lg flex', getAlertType(message.type)]"
      role="alert"
    >
      <div class="flex-grow">
        <h3 class="font-bold" v-if="message.type !== 'Success'">{{ message.type }}</h3>
        <div class="text-xs">{{ message.message }}</div>
      </div>
      <div>
        <button class="btn btn-sm btn-circle" @click="dismissMessage(index)">
          ✕
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useMessagesStore } from '../stores/messagesStore'

const messages = useMessagesStore()

function getAlertType(type: string) {
  switch (type) {
    case 'InvalidInput':
      return 'alert-warning'
    case 'Database':
    case 'Printer':
    case 'Internal':
    case 'MutexPoisoned':
      return 'alert-error'
    case 'Success':
      return 'alert-success'
    default:
      return 'alert-error'
  }
}

function dismissMessage(index: number) {
  messages.removeMessage(index)
}
</script>
