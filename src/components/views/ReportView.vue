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
            <div class="text-4xl opacity-50">€</div>
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
            <div class="text-4xl opacity-50">💵</div>
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
            <div class="text-4xl opacity-50">💳</div>
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
            <div class="text-4xl opacity-50">↩️</div>
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
  )
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
    }>("get_transactions_recap");
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

  if (
    invoiceSalesData.value.length === 0 &&
    productSalesData.value.length === 0
  ) {
    messages.addInvalidInput($t("reports-messages-no-data-available"));
  }
};

onMounted(() => {
  // Default to today
  setToday();
});
</script>
