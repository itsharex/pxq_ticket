<template>
    <div class="p-4">
      <h2 class="text-2xl font-bold mb-4">待支付</h2>
      <div v-if="loadingOngoing" class="text-center">加载中...</div>
      <div v-else-if="errorOngoing" class="text-center text-red-500">{{ errorOngoing }}</div>
      <div v-else>
        <div v-if="ongoingOrders.length === 0" class="text-center text-gray-500">暂无进行中的订单</div>
        <div v-else>
          <div v-for="order in ongoingOrders" :key="order.id" class="border p-4 mb-4">
            <h3 class="text-lg font-bold">订单号: {{ order.id }}</h3>
            <div class="flex items-center mb-2" v-for="item in order.items" :key="item.id">
              <img :src="item.image" alt="Product Image" class="w-20 h-20 mr-4 rounded-md">
              <div>
                <h4 class="text-md font-bold">{{ item.title }}</h4>
                <p>数量: {{ item.quantity }}</p>
                <p>价格: {{ item.price }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
      <h2 class="text-2xl font-bold mb-4">历史订单</h2>
      <div v-if="loadingHistory" class="text-center">加载中...</div>
      <div v-else-if="errorHistory" class="text-center text-red-500">{{ errorHistory }}</div>
      <div v-else>
        <div v-if="historyOrders.length === 0" class="text-center text-gray-500">暂无历史订单</div>
        <div v-else>
          <div v-for="order in historyOrders" :key="order.id" class="border p-4 mb-4">
            <h3 class="text-lg font-bold">订单号: {{ order.id }}</h3>
            <div class="flex items-center mb-2" v-for="item in order.items" :key="item.id">
              <img :src="item.image" alt="Product Image" class="w-20 h-20 mr-4 rounded-md">
              <div>
                <h4 class="text-md font-bold">{{ item.title }}</h4>
                <p>数量: {{ item.quantity }}</p>
                <p>价格: {{ item.price }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent } from 'vue';
import { Order } from '../types';
  

  
  export default defineComponent({
    name: 'OrderPage',
    data() {
      return {
        loadingOngoing: false,
        errorOngoing: '',
        ongoingOrders: [] as Order[],
        loadingHistory: false,
        errorHistory: '',
        historyOrders: [] as Order[]
      };
    },
    mounted() {
      this.fetchOngoingOrders();
      this.fetchHistoryOrders();
    },
    methods: {
      fetchOngoingOrders() {
        // 模拟加载进行中的订单
        this.loadingOngoing = true;
        setTimeout(() => {
          this.loadingOngoing = false;
          // 模拟数据
          this.ongoingOrders = [
            {
              id: 1,
              items: [
                { id: 1, image: 'https://via.placeholder.com/100', title: '商品1', quantity: 2, price: 100 },
                { id: 2, image: 'https://via.placeholder.com/100', title: '商品2', quantity: 1, price: 50 }
              ]
            },
            {
              id: 2,
              items: [
                { id: 3, image: 'https://via.placeholder.com/100', title: '商品3', quantity: 3, price: 80 }
              ]
            }
          ];
        }, 1000);
      },
      fetchHistoryOrders() {
        // 模拟加载历史订单
        this.loadingHistory = true;
        setTimeout(() => {
          this.loadingHistory = false;
          // 模拟数据
          this.historyOrders = [
            {
              id: 3,
              items: [
                { id: 4, image: 'https://via.placeholder.com/100', title: '商品4', quantity: 1, price: 120 },
                { id: 5, image: 'https://via.placeholder.com/100', title: '商品5', quantity: 2, price: 90 }
              ]
            },
            {
              id: 4,
              items: [
                { id: 6, image: 'https://via.placeholder.com/100', title: '商品6', quantity: 1, price: 60 }
              ]
            }
          ];
        }, 1000);
      }
    }
  });
  </script>
  
  <style scoped>
  /* Add custom styles here */
  </style>
  