<template>
  <div class="p-4 md:p-6">
    <h1 class="text-2xl font-bold mb-4">
      {{ t('reports.sales_report.title') }}
    </h1>

    <div class="overflow-x-auto">
      <table class="table table-zebra w-full">
        <thead>
          <tr>
            <th>{{ t('reports.sales_report.product_name') }}</th>
            <th>{{ t('reports.sales_report.quantity_sold') }}</th>
            <th>{{ t('reports.sales_report.value_sold') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="item in salesData" :key="item.product_id">
            <td>{{ item.product_name }}</td>
            <td>{{ item.total_quantity_sold }}</td>
            <td>{{ formatCurrency(item.total_value_sold) }}</td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-if="loading" class="loading">
      {{ t('reports.messages.loading_data') }}
    </div>
    <div v-if="salesData.length === 0" class="alert alert-warning">
      {{ t('reports.messages.no_data_available') }}
    </div>

    <div class="pt-4">
      <button class="btn btn-error" @click="clearHistory()">
        {{ t('reports.clear_reports_button') }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useMessagesStore } from '../../stores/messagesStore'

interface ItemSale {
  product_id: number,
  product_name: string,
  total_quantity_sold: number,
  total_value_sold: number
}

const { t } = useI18n()
const messages = useMessagesStore()
const loading = ref<boolean>(true)
const salesData = ref<ItemSale[]>([])

const formatCurrency = (value: number) => {
  return new Intl.NumberFormat('it-IT', { style: 'currency', currency: 'EUR' }).format(value);
}

const clearHistory = async () => {
  try {
    await invoke('clear_sales_data')

    salesData.value = []
  } catch (err) {
    messages.addUnknownError(err)
  }
}

onMounted(async function () {
  try {
    const data = await invoke<ItemSale[]>('get_sales_recap');
    salesData.value = data.sort(function (a, b) {
      if (a < b) {
        return -1;
      } else if (a > b) {
        return 1;
      } else {
        return 0;
      }
    })

    loading.value = false
  } catch (err) {
    messages.addUnknownError(err)
  }
})

</script>
