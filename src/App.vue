<script setup lang="ts">
import { onMounted } from 'vue';
import { get_value, set_value } from './store';
import router from './router';

onMounted(async ()=>{
  const access_token = await get_value('access_token')
  if (access_token == null) {
    set_value("access_token", null)
    router.push('login')
  }else{
    console.log(access_token)
    router.push('show')
  }
})
</script>

<template>
  <router-view></router-view>
</template>