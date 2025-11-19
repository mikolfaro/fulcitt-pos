<template>
  <div class="p-4 md:p-6">
    <h1 class="text-2xl font-bold mb-4">
      {{ $t('reports-title') }}
    </h1>

    <div class="overflow-x-auto pb-8">
      <h2 class="pb-4">
        {{ $t('reports-sales-by-product-title') }}
      </h2>

      <table class="table table-zebra w-full">
        <thead>
          <tr>
            <th>{{ $t('reports-sales-by-product-product-name') }}</th>
            <th class="text-right">{{ $t('reports-sales-by-product-quantity-sold') }}</th>
            <th class="text-right">{{ $t('reports-sales-by-product-value-sold') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="item in productSalesData" :key="item.product_id">
            <td>{{ item.product_name }}</td>
            <td class="text-right">{{ item.total_quantity_sold }}</td>
            <td class="text-right">{{ formatCurrency(item.total_value_sold) }}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="overflow-x-auto pb-8">
      <h2 class="pb-4">
        {{ $t('reports-refunds-title') }}
      </h2>

      <table class="table table-zebra w-full">
        <thead>
          <tr>
            <th>{{ $t('reports-refunds-product-name') }}</th>
            <th class="text-right">{{ $t('reports-refunds-quantity-refunded') }}</th>
            <th class="text-right">{{ $t('reports-refunds-value-refunded') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="item in refundsData" :key="item.product_id">
            <td>{{ item.product_name }}</td>
            <td class="text-right">{{ item.total_quantity_sold }}</td>
            <td class="text-right">{{ formatCurrency(item.total_value_sold) }}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="overflow-x-auto pb-8">
      <h2 class="pb-4">
        {{ $t('reports-today-sales-title') }}
      </h2>

      <table class="table table-zebra w-full">
        <thead>
          <tr>
            <th class="text-right">{{ $t('reports-today-sales-invoice-number') }}</th>
            <th>{{ $t('reports-today-sales-sales-time') }}</th>
            <th class="text-right">{{ $t('reports-today-sales-amount-sold') }}</th>
            <th>{{ $t('reports-today-sales-payment-method') }}</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="sale in invoiceSalesData" :key="sale.id">
            <td class="text-right">{{ sale.number || 'N/A' }}</td>
            <td>{{ formatTime(sale.sale_time) }}</td>
            <td class="text-right">{{ formatCurrency(sale.total_amount) }}</td>
            <td>-</td>
            <td>
              <button class="btn btn-xs btn-outline btn-primary" @click="printTicket(sale)">
                {{ $t('reports-today-sales-reprint-tickets-button') }}
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="loading" class="loading">
      {{ $t('reports-messages-loading-data') }}
    </div>

    <div class="flex justify-between gap-4 pt-4">
      <button class="btn btn-primary" @click="exportXlsx()">
        {{ $t('reports-export-xlsx-button') }}
      </button>

      <button class="btn btn-error" @click="clearHistory()">
        {{ $t('reports-clear-reports-button') }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted } from 'vue'
import { useFluent } from 'fluent-vue'
import { useMessagesStore } from '../../stores/messagesStore'

interface ItemSale {
  product_id: number,
  product_name: string,
  total_quantity_sold: number,
  total_value_sold: number
}

interface Sale {
  id: string,
  number: number | null,
  sale_time: Date,
  total_amount: number,
  payment_method: string

}

const { $t } = useFluent()
const messages = useMessagesStore()
const loading = ref<boolean>(true)
const productSalesData = ref<ItemSale[]>([])
const refundsData = ref<ItemSale[]>([])
const invoiceSalesData = ref<Sale[]>([])

const currencyFormatter = new Intl.NumberFormat('it-IT', { style: 'currency', currency: 'EUR' })
const formatCurrency = (value: number) => {
  return currencyFormatter.format(value)
}

const timeFormatter = new Intl.DateTimeFormat('it-IT', { timeStyle: 'short' })
const formatTime = (value: Date) => {
  return timeFormatter.format(value)
}

const printTicket = async (sale: Sale) => {
  try {
    await invoke('print_sale', { saleId: sale.id })
  } catch (err) {
    messages.addUnknownError(err)
  }
}

const clearHistory = async () => {
  try {
    await invoke('clear_sales_data')

    productSalesData.value = []
    invoiceSalesData.value = []
    refundsData.value = []
  } catch (err) {
    messages.addUnknownError(err)
  }
}

const exportXlsx = async () => {
  try {
    await invoke('export_sales')
    messages.addSuccess($t('reports-messages-export-xlsx-completed'))
  } catch (err) {
    messages.addUnknownError(err)
  }
}

interface WithProductName { product_name: string }
const sortByProductName = (a: WithProductName, b: WithProductName) => {
  if (a.product_name < b.product_name) {
    return -1
  } else if (a.product_name > b.product_name) {
    return 1
  } else {
    return 0
  }
}

onMounted(async function () {
  try {
    const { sales, refunds } = await invoke<{ sales: ItemSale[], refund: ItemSale[] }>('get_transactions_recap')
    productSalesData.value = sales.sort(sortByProductName)
    refundsData.value = refunds.sort(sortByProductName)

    loading.value = false
  } catch (err) {
    messages.addUnknownError(err)
  }

  try {
    type DeserializedSale = Omit<Sale, 'sale_time'> & { sale_time: string };
    const data = await invoke<DeserializedSale[]>("get_today_sales")

    invoiceSalesData.value = data.map(function (sale) {
      return { ...sale, sale_time: new Date(sale.sale_time) }
    }).sort(function (a, b) {
      // Sort by date DESC (most recent first)
      if (a.sale_time.getTime() !== b.sale_time.getTime()) {
        return b.sale_time.getTime() - a.sale_time.getTime()
      }
      // If dates are equal, sort by sale number DESC
      const aNum = a.number ?? 0
      const bNum = b.number ?? 0
      return bNum - aNum
    })
  } catch (err) {
    messages.addUnknownError(err)
  }

  if (invoiceSalesData.value.length === 0 && productSalesData.value.length === 0) {
    messages.addInvalidInput($t('reports-messages-no-data-available'))
  }
})

</script>
