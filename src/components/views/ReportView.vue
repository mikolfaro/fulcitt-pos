<template>
  <div class="p-4 md:p-6">
    <h1 class="text-2xl font-bold mb-4">Sales report</h1>

    <div class="overflow-x-auto">
      <table class="table table-zebra w-full">
        <thead>
          <tr>
            <th>Product Name</th>
            <th>Total Quantity Sold</th>
            <th>Total Value Sold</th>
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
    <div v-if="loading" class="loading">Loading data...</div>
    <div v-if="salesData.length === 0" class="alert alert-warning">No sales data available.</div>

    <div class="pt-4">
      <button class="btn btn-error" @click="clearHistory()">Clear sales history</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted } from 'vue';
import { useMessagesStore } from '../../stores/messagesStore';

interface ItemSale {
  product_id: number,
  product_name: string,
  total_quantity_sold: number,
  total_value_sold: number
}

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
