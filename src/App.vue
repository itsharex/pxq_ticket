<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue';
import { get_value, set_value } from './store';
import router from './router';
import { toast } from 'vue3-toastify';
import { invoke } from '@tauri-apps/api';
import { GetUserLocationResult, RefreshTokenResult } from './types';

const timer = ref(0);


onMounted(async () => {
  try {
    const access_token = await get_value('access_token')
    if (access_token == null) {
      set_value("access_token", "")
      router.push('login')
    } else {
      router.push('show')
    }
  } catch (error: any) {
    toast.error(error)
  }

  try {

    const user_location_res: GetUserLocationResult = await invoke("get_user_location")
    console.log("获取用户地区: ", user_location_res)
    if (user_location_res.statusCode == 200) {
        await set_value("user_location", user_location_res.data)
    }
  } catch (error: any) {
      toast.error("获取用户地区失败:" + error);
  }
  timer.value = setInterval(async () => {
    const res: RefreshTokenResult = await invoke('refresh_token');
    if (res.statusCode == 200) {
        console.log(`刷新token:${res}`)
    }else {
      console.log("刷新失败:", res);
    }
  }, 1000 * 60 * 30);
})


onBeforeUnmount(async () => {
  clearInterval(timer.value);
})
</script>

<template>
  <router-view></router-view>
</template>