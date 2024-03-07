<template>
    <div class="p-4">
      <div class="flex justify-between items-center">
    <h2 class="text-2xl font-bold mb-4">待支付</h2>
    <button @click="refresh" class="btn glass">刷新</button>
</div>
      
      <div v-if="loadingOngoing" class="text-center">加载中...</div>
      <div v-else-if="errorOngoing" class="text-center text-red-500">{{ errorOngoing }}</div>
      <div v-else>
        <div v-if="ongoingOrders.length === 0" class="text-center text-gray-500">暂无进行中的订单</div>
        <div v-else>
          <div v-for="order in ongoingOrders" :key="order.orderId" class="border p-4 mb-4">
            <h3 class="text-lg font-bold">订单号: {{ order.orderNumber }}</h3>
            <div class="flex items-center mb-2">
              <img :src="order.displayPosterURL" alt="Product Image" class="w-20 h-20 mr-4 rounded-md">
              <div>
                <h4 class="text-md font-bold">{{ order.firstShowName }}</h4>
                <p>场次: {{ order.firstSessionName }}</p>
                <p>数量: {{ order.qty }}</p>
                <p>价格: {{ order.payAmount }}</p>
                <p>状态: {{ order.orderDetailState.displayName }}</p>
              </div>
            </div>

          </div>
        </div>
      </div>
        <div class="flex justify-between items-center">
      <h2 class="text-2xl font-bold mb-4">待支付</h2>
  </div>
      <div v-if="loadingHistory" class="text-center">加载中...</div>
      <div v-else-if="errorHistory" class="text-center text-red-500">{{ errorHistory }}</div>
      <div v-else>
        <div v-if="historyOrders.length === 0" class="text-center text-gray-500">暂无历史订单</div>
        <div v-else>
          <div v-for="order in historyOrders" :key="order.orderId" class="border p-4 mb-4">
            <h3 class="text-lg font-bold">订单号: {{ order.orderNumber }}</h3>
            <div class="flex items-center mb-2">
              <img :src="order.displayPosterURL" alt="Product Image" class="w-20 h-20 mr-4 rounded-md">
              <div>
                <h4 class="text-md font-bold">{{ order.firstShowName }}</h4>
                <p>场馆: {{ order.firstVenueName }}</p>
                <p>场次: {{ order.firstSessionName }}</p>
                <p>数量: {{ order.qty }}</p>
                <p>价格: {{ order.payAmount }}</p>
                <p>状态: {{ order.orderDetailState.displayName }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </template>
  
<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { GetOrdersResult, Order } from '../types';
import { toast } from 'vue3-toastify';
  
const loadingOngoing = ref(false)
const loadingHistory = ref(false)
const ongoingOrders = ref([] as Order[]);
const historyOrders = ref([] as Order[])
const errorHistory = ref("")
const errorOngoing = ref("")

const fetchOngoingOrderList = async () => {
  try {
    loadingOngoing.value = true
    const res: GetOrdersResult = await invoke('get_pending_orders')
    if (res.statusCode == 200) {
      ongoingOrders.value = res.data
    }
  console.log("订单:", res)
  } catch (error: any) {
    console.log(error)
  }finally{
    loadingOngoing.value = false
  }
}


const fetchHistorOrderList = async () => {
  try {
    loadingHistory.value = true
    const res: GetOrdersResult = await invoke('get_terminate_orders')
    if (res.statusCode == 200) {
      historyOrders.value = res.data
    }
  console.log("订单:", res)
  } catch (error: any) {
    console.log(error)
  }finally{
    loadingHistory.value = false
  }
}

const refresh = async () => {
  try {
    await fetchOngoingOrderList()
    await fetchHistorOrderList()
  } catch (error: any) {
    toast.error(error)
  }
}


onMounted(async () => {
  await fetchHistorOrderList()
  await fetchOngoingOrderList()
})

  </script>
  
  <style scoped>
  /* Add custom styles here */
  </style>
  