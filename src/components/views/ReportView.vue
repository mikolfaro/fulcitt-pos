<template>
  <div class="p-4 md:p-6">
    <h1 class="text-2xl font-bold mb-4">
      {{ $t('reports-title') }}
    </h1>

    <!-- Date Range Filter -->
    <div class="card bg-base-200 mb-6">
      <div class="card-body">
        <h3 class="card-title text-lg mb-4">{{ $t('reports-date-filter-title') }}</h3>
        <div class="flex flex-wrap gap-4 items-end">
          <div class="form-control">
            <label class="label">
              <span class="label-text">{{ $t('reports-date-start-label') }}</span>
            </label>
            <input v-model="startDate" type="date" class="input input-bordered" @change="loadData" />
          </div>
          <div class="form-control">
            <label class="label">
              <span class="label-text">{{ $t('reports-date-end-label') }}</span>
            </label>
            <input v-model="endDate" type="date" class="input input-bordered" @change="loadData" />
          </div>
          <div class="flex gap-2">
            <button class="btn btn-sm btn-outline" @click="setToday">
              {{ $t('reports-date-today') }}
            </button>
            <button class="btn btn-sm btn-outline" @click="setLast7Days">
              {{ $t('reports-date-last-7-days') }}
            </button>
            <button class="btn btn-sm btn-outline" @click="setThisMonth">
              {{ $t('reports-date-this-month') }}
            </button>
            <button class="btn btn-sm btn-outline" @click="setAllTime">
              {{ $t('reports-date-all-time') }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Statistics Cards -->
    <div class="grid grid-cols-4 gap-4 mb-6">
      <!-- Total Sales -->
      <div class="card bg-primary text-primary-content">
        <div class="card-body">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm opacity-80">{{ $t('reports-stat-total-sales') }}</p>
              <p class="text-3xl font-bold">{{ formatCurrency(totalAmountSold - totalRefundsAmount) }}</p>
            </div>
            <div class="text-4xl opacity-50">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                stroke="currentColor" class="w-14 h-14">
                <path stroke-linecap="round" stroke-linejoin="round"
                  d="M14.25 7.756a4.5 4.5 0 1 0 0 8.488M7.5 10.5h5.25m-5.25 3h5.25M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
              </svg>
            </div>
          </div>
          <p class="text-sm mt-2">{{ $t('reports-stat-transactions-count', { count: invoiceSalesData.length }) }}</p>
        </div>
      </div>

      <!-- Cash Payments -->
      <div class="card bg-base-200">
        <div class="card-body">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm opacity-80">{{ $t('reports-stat-cash-payments') }}</p>
              <p class="text-3xl font-bold">{{ formatCurrency(cashTotal) }}</p>
            </div>
            <div class="text-4xl opacity-50">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                stroke="currentColor" class="w-12 h-12">
                <path strokeLinecap="round" strokeLinejoin="round"
                  d="M2.25 18.75a60.07 60.07 0 0 1 15.797 2.101c.727.198 1.453-.342 1.453-1.096V18.75M3.75 4.5v.75A.75.75 0 0 1 3 6h-.75m0 0v-.375c0-.621.504-1.125 1.125-1.125H20.25M2.25 6v9m18-10.5v.75c0 .414.336.75.75.75h.75m-1.5-1.5h.375c.621 0 1.125.504 1.125 1.125v9.75c0 .621-.504 1.125-1.125 1.125h-.375m1.5-1.5H21a.75.75 0 0 0-.75.75v.75m0 0H3.75m0 0h-.375a1.125 1.125 0 0 1-1.125-1.125V15m1.5 1.5v-.75A.75.75 0 0 0 3 15h-.75M15 10.5a3 3 0 1 1-6 0 3 3 0 0 1 6 0Zm3 0h.008v.008H18V10.5Zm-12 0h.008v.008H6V10.5Z" />
              </svg>
            </div>
          </div>
          <p class="text-sm mt-2">{{ $t('reports-stat-cash-count', { count: cashCount }) }}</p>
        </div>
      </div>

      <!-- Card Payments -->
      <div class="card bg-base-200">
        <div class="card-body">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm opacity-80">{{ $t('reports-stat-card-payments') }}</p>
              <p class="text-3xl font-bold">{{ formatCurrency(cardTotal) }}</p>
            </div>

            <div class="text-4xl opacity-50">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                stroke="currentColor" class="w-14 h-14">
                <path stroke-linecap="round" stroke-linejoin="round"
                  d="M2.25 8.25h19.5M2.25 9h19.5m-16.5 5.25h6m-6 2.25h3m-3.75 3h15a2.25 2.25 0 0 0 2.25-2.25V6.75A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25v10.5A2.25 2.25 0 0 0 4.5 19.5Z" />
              </svg>
            </div>
          </div>
          <p class="text-sm mt-2">{{ $t('reports-stat-card-count', { count: cardCount }) }}</p>
        </div>
      </div>

      <!-- Refunds -->
      <div class="card bg-warning text-warning-content">
        <div class="card-body">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm opacity-80">{{ $t('reports-stat-total-refunds') }}</p>
              <p class="text-3xl font-bold">{{ formatCurrency(totalRefundsAmount) }}</p>
            </div>
            <div class="text-4xl opacity-50">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                stroke="currentColor" class="w-14 h-14">
                <path stroke-linecap="round" stroke-linejoin="round"
                  d="M8.25 9.75h4.875a2.625 2.625 0 0 1 0 5.25H12M8.25 9.75 10.5 7.5M8.25 9.75 10.5 12m9-7.243V21.75l-3.75-1.5-3.75 1.5-3.75-1.5-3.75 1.5V4.757c0-1.108.806-2.057 1.907-2.185a48.507 48.507 0 0 1 11.186 0c1.1.128 1.907 1.077 1.907 2.185Z" />
              </svg>
            </div>
          </div>
          <p class="text-sm mt-2">{{ $t('reports-stat-refunds-count', { count: totalRefundsCount }) }}</p>
        </div>
      </div>
    </div>

    <!-- Sales by Product Table -->
    <div class="card bg-base-200 mb-6">
      <div class="card-body">
        <h2 class="card-title text-lg mb-4">
          {{ $t('reports-sales-by-product-title') }}
        </h2>
        <div class="overflow-x-auto">
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
              <tr v-if="productSalesData.length === 0" class="text-center text-base-content/50">
                <td colspan="3">{{ $t('reports-no-data') }}</td>
              </tr>
            </tbody>
            <tfoot v-if="productSalesData.length > 0" class="font-bold bg-base-300">
              <tr>
                <td>{{ $t('reports-total') }}</td>
                <td class="text-right">{{ totalItemsSold }}</td>
                <td class="text-right">{{ formatCurrency(totalProductsValue) }}</td>
              </tr>
            </tfoot>
          </table>
        </div>
      </div>
    </div>

    <!-- Refunds Table -->
    <div class="card bg-base-200 mb-6" v-if="refundsData.length > 0">
      <div class="card-body">
        <h2 class="card-title text-lg mb-4">
          {{ $t('reports-refunds-title') }}
        </h2>
        <div class="overflow-x-auto">
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
            <tfoot class="font-bold bg-base-300">
              <tr>
                <td>{{ $t('reports-total') }}</td>
                <td class="text-right">{{ totalRefundsCount }}</td>
                <td class="text-right">{{ formatCurrency(totalRefundsAmount) }}</td>
              </tr>
            </tfoot>
          </table>
        </div>
      </div>
    </div>

    <!-- Today Sales Table -->
    <div class="card bg-base-200 mb-6">
      <div class="card-body">
        <h2 class="card-title text-lg mb-4">
          {{ $t('reports-today-sales-title') }}
        </h2>
        <div class="overflow-x-auto">
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
                <td>{{ formatDateTime(sale.sale_time) }}</td>
                <td class="text-right">{{ formatCurrency(sale.total_amount) }}</td>
                <td>{{ $t(`reports-payment-method-${sale.payment_method.toLowerCase()}`) }}</td>
                <td>
                  <button class="btn btn-xs btn-outline btn-primary" @click="printTicket(sale)">
                    {{ $t('reports-today-sales-reprint-tickets-button') }}
                  </button>
                </td>
              </tr>
              <tr v-if="invoiceSalesData.length === 0" class="text-center text-base-content/50">
                <td colspan="5">{{ $t('reports-no-data') }}</td>
              </tr>
            </tbody>
            <tfoot v-if="invoiceSalesData.length > 0" class="font-bold bg-base-300">
              <tr>
                <td colspan="2">{{ $t('reports-total') }}</td>
                <td class="text-right">{{ formatCurrency(totalAmountSold) }}</td>
                <td colspan="2"></td>
              </tr>
            </tfoot>
          </table>
        </div>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="flex justify-center items-center py-12">
      <span class="loading loading-spinner loading-lg text-primary"></span>
      <span class="ml-4 text-lg">{{ $t('reports-messages-loading-data') }}</span>
    </div>

    <!-- Action Buttons -->
    <div class="flex flex-wrap justify-between gap-4 pt-4">
      <button class="btn btn-primary" @click="exportXlsx()" :disabled="loading || invoiceSalesData.length === 0">
        {{ $t('reports-export-xlsx-button') }}
      </button>

      <button class="btn btn-error" @click="clearHistory()"
        :disabled="loading || (invoiceSalesData.length === 0 && productSalesData.length === 0)">
        {{ $t('reports-clear-reports-button') }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { useFluent } from "fluent-vue";
import { computed, onMounted, ref } from "vue";
import { useMessagesStore } from "../../stores/messagesStore";

interface ItemSale {
  product_id: number;
  product_name: string;
  total_quantity_sold: number;
  total_value_sold: number;
}

interface Sale {
  id: string;
  number: number | null;
  sale_time: Date;
  total_amount: number;
  payment_method: string;
}

const { $t } = useFluent();
const messages = useMessagesStore();
const loading = ref<boolean>(true);
const productSalesData = ref<ItemSale[]>([]);
const refundsData = ref<ItemSale[]>([]);
const invoiceSalesData = ref<Sale[]>([]);

// Date range filters
const startDate = ref<string>("");
const endDate = ref<string>("");

// Statistics computed properties
const totalAmountSold = computed(() => {
  return invoiceSalesData.value.reduce(
    (sum, sale) => sum + sale.total_amount,
    0,
  );
});

const totalItemsSold = computed(() => {
  return productSalesData.value.reduce(
    (sum, item) => sum + item.total_quantity_sold,
    0,
  );
});

const totalProductsValue = computed(() => {
  return productSalesData.value.reduce(
    (sum, item) => sum + item.total_value_sold,
    0,
  );
});

const totalRefundsAmount = computed(() => {
  return refundsData.value.reduce(
    (sum, item) => sum + item.total_value_sold,
    0,
  );
});

const totalRefundsCount = computed(() => {
  return refundsData.value.reduce(
    (sum, item) => sum + item.total_quantity_sold,
    0,
  );
});

const cashTotal = computed(() => {
  return invoiceSalesData.value
    .filter((sale) => sale.payment_method.toLowerCase() === "cash")
    .reduce((sum, sale) => sum + sale.total_amount, 0);
});

const cashCount = computed(() => {
  return invoiceSalesData.value.filter(
    (sale) => sale.payment_method.toLowerCase() === "cash",
  ).length;
});

const cardTotal = computed(() => {
  return invoiceSalesData.value
    .filter((sale) => sale.payment_method.toLowerCase() === "card")
    .reduce((sum, sale) => sum + sale.total_amount, 0);
});

const cardCount = computed(() => {
  return invoiceSalesData.value.filter(
    (sale) => sale.payment_method.toLowerCase() === "card",
  ).length;
});

const currencyFormatter = new Intl.NumberFormat("it-IT", {
  style: "currency",
  currency: "EUR",
});
const formatCurrency = (value: number) => {
  return currencyFormatter.format(value);
};

const dateTimeFormatter = new Intl.DateTimeFormat("it-IT", {
  dateStyle: "short",
  timeStyle: "short",
});
const formatDateTime = (value: Date) => {
  return dateTimeFormatter.format(value);
};

const printTicket = async (sale: Sale) => {
  try {
    await invoke("print_sale", { saleId: sale.id });
  } catch (err) {
    messages.addUnknownError(err);
  }
};

const clearHistory = async () => {
  try {
    await invoke("clear_sales_data");

    productSalesData.value = [];
    invoiceSalesData.value = [];
    refundsData.value = [];
    messages.addSuccess($t("reports-messages-data-cleared"));
  } catch (err) {
    messages.addUnknownError(err);
  }
};

const exportXlsx = async () => {
  try {
    await invoke("export_sales");
    messages.addSuccess($t("reports-messages-export-xlsx-completed"));
  } catch (err) {
    messages.addUnknownError(err);
  }
};

interface WithProductName {
  product_name: string;
}
const sortByProductName = (a: WithProductName, b: WithProductName) => {
  if (a.product_name < b.product_name) {
    return -1;
  } else if (a.product_name > b.product_name) {
    return 1;
  } else {
    return 0;
  }
};

// Date filter functions
const formatDateForInput = (date: Date): string => {
  return date.toISOString().split("T")[0];
};

const setToday = () => {
  const today = new Date();
  startDate.value = formatDateForInput(today);
  endDate.value = formatDateForInput(today);
  loadData();
};

const setLast7Days = () => {
  const end = new Date();
  const start = new Date();
  start.setDate(start.getDate() - 6);
  startDate.value = formatDateForInput(start);
  endDate.value = formatDateForInput(end);
  loadData();
};

const setThisMonth = () => {
  const now = new Date();
  const start = new Date(now.getFullYear(), now.getMonth(), 1);
  const end = new Date(now.getFullYear(), now.getMonth() + 1, 0);
  startDate.value = formatDateForInput(start);
  endDate.value = formatDateForInput(end);
  loadData();
};

const setAllTime = () => {
  startDate.value = "";
  endDate.value = "";
  loadData();
};

const loadData = async () => {
  loading.value = true;

  try {
    const { sales, refunds } = await invoke<{
      sales: ItemSale[];
      refunds: ItemSale[];
    }>("get_transactions_recap_by_date_range", {
      startDate: startDate.value,
      endDate: endDate.value,
    });
    productSalesData.value = sales.sort(sortByProductName);
    refundsData.value = refunds.sort(sortByProductName);
  } catch (err) {
    messages.addUnknownError(err);
  }

  try {
    type DeserializedSale = Omit<Sale, "sale_time"> & { sale_time: string };
    let data: DeserializedSale[];

    // Use date range if specified
    if (startDate.value && endDate.value) {
      data = await invoke<DeserializedSale[]>("get_sales_by_date_range", {
        startDate: startDate.value,
        endDate: endDate.value,
      });
    } else {
      data = await invoke<DeserializedSale[]>("get_all_sales");
    }

    invoiceSalesData.value = data
      .map(function (sale) {
        return { ...sale, sale_time: new Date(sale.sale_time) };
      })
      .sort(function (a, b) {
        // Sort by date DESC (most recent first)
        if (a.sale_time.getTime() !== b.sale_time.getTime()) {
          return b.sale_time.getTime() - a.sale_time.getTime();
        }
        // If dates are equal, sort by sale number DESC
        const aNum = a.number ?? 0;
        const bNum = b.number ?? 0;
        return bNum - aNum;
      });
  } catch (err) {
    messages.addUnknownError(err);
  }

  loading.value = false;
};

onMounted(() => {
  // Default to today
  setToday();
});
</script>
