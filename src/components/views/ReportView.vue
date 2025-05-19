<template>
  <div class="p-4 md:p-6">
    <h1 class="text-2xl font-bold mb-4">
      {{ t('reports.title') }}
    </h1>

    <div class="overflow-x-auto pb-8">
      <h2 class="pb-4">
        {{ t('reports.sales_by_product.title') }}
      </h2>

      <table class="table table-zebra w-full">
        <thead>
          <tr>
            <th>{{ t('reports.sales_by_product.product_name') }}</th>
            <th>{{ t('reports.sales_by_product.quantity_sold') }}</th>
            <th>{{ t('reports.sales_by_product.value_sold') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="item in productSalesData"
            :key="item.product_id"
          >
            <td>{{ item.product_name }}</td>
            <td>{{ item.total_quantity_sold }}</td>
            <td>{{ formatCurrency(item.total_value_sold) }}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="overflow-x-auto pb-8">
      <h2 class="pb-4">
        {{ t('reports.today_sales.title') }}
      </h2>

      <table class="table table-zebra w-full">
        <thead>
          <tr>
            <th>{{ t('reports.today_sales.invoice_number') }}</th>
            <th>{{ t('reports.today_sales.sales_time') }}</th>
            <th>{{ t('reports.today_sales.amount_sold') }}</th>
            <th>{{ t('reports.today_sales.payment_method') }}</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="sale in invoiceSalesData"
            :key="sale.id"
          >
            <td>{{ sale.id }}</td>
            <td>{{ formatTime(sale.sale_time) }}</td>
            <td>{{ formatCurrency(sale.total_amount) }}</td>
            <td>-</td>
            <td>
              <button class="btn btn-xs btn-outline btn-primary">
                {{ t('reports.today_sales.reprint_tickets_button') }}
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="loading" class="loading">
      {{ t('reports.messages.loading_data') }}
    </div>
    <div v-if="productSalesData.length === 0" class="alert alert-warning">
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

interface Sale {
  id: number,
  sale_time: Date,
  total_amount: number,
  payment_method: string

}

const { t } = useI18n()
const messages = useMessagesStore()
const loading = ref<boolean>(true)
const productSalesData = ref<ItemSale[]>([])
const invoiceSalesData = ref<Sale[]>([])

const currencyFormatter = new Intl.NumberFormat('it-IT', { style: 'currency', currency: 'EUR' })
const formatCurrency = (value: number) => {
  return currencyFormatter.format(value)
}

const timeFormatter = new Intl.DateTimeFormat('it-IT', { timeStyle: 'short' })
const formatTime = (value: Date) => {
  return timeFormatter.format(value)
}

const clearHistory = async () => {
  try {
    await invoke('clear_sales_data')

    productSalesData.value = []
    invoiceSalesData.value = []
  } catch (err) {
    messages.addUnknownError(err)
  }
}

onMounted(async function () {
  try {
    const data = await invoke<ItemSale[]>('get_sales_recap')
    productSalesData.value = data.sort(function (a, b) {
      if (a.product_name < b.product_name) {
        return -1
      } else if (a.product_name > b.product_name) {
        return 1
      } else {
        return 0
      }
    })

    loading.value = false
  } catch (err) {
    messages.addUnknownError(err)
  }

  try {
    const data = await invoke("get_today_sales")
    console.log("get_today_sales", data)
    invoiceSalesData.value = data.map(function (sale) {
      return { ...sale, sale_time: Date.parse(sale.sale_time) }
    })
  } catch (err) {
    console.error(err)
  }
})

</script>
