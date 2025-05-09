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
    <div v-else-if="error" class="text-error">{{ error }}</div>
    <div v-else-if="salesData.length === 0" class="alert alert-warning">No sales data available.</div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted } from 'vue';

interface ItemSale {
  product_id: number,
  product_name: string,
  total_quantity_sold: number,
  total_value_sold: number
}

const loading = ref<boolean>(true)
const error = ref<string>('')

const salesData = ref<ItemSale[]>([]);

const formatCurrency = (value) => {
  return new Intl.NumberFormat('it-IT', { style: 'currency', currency: 'EUR' }).format(value);
};

onMounted(async function () {
  const data = await invoke('get_sales_recap');
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
});

</script>
