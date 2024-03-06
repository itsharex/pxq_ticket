<script setup lang="ts">
import { invoke, shell } from "@tauri-apps/api"
import { onMounted } from "vue";
import { RouterLink, RouterView } from "vue-router";
import { UserProfileResult, RefreshTokenResult } from '../types'
import { ref } from 'vue'
import { toast } from "vue3-toastify";
import router from "../router";
import { set_value } from "../store";

const avatarSrc = ref("")
const nickname = ref("")

const onClickGithub = () => {
  shell.open('https://github.com/ClassmateLin/pxq_ticket')
}

onMounted(async ()=>{
  console.log("hello")
  try {
    const res: UserProfileResult = await invoke("get_user_profile");
    console.log(res)
    if (res.statusCode == 200) {
      avatarSrc.value = res.data.avatar
      nickname.value = res.data.nickname
      toast.success(`用户:${res.data.nickname}登录成功`)
      return
    }else{
      const res: RefreshTokenResult = await invoke('refresh_token');
      console.log(res)
      if (res.statusCode == 200) {
        await set_value("access_token", res.data.accessToken)
        await set_value("refresh_token", res.data.refreshToken)
        toast.success("刷新AccessToken成功")
        router.push('show')
      }else{
        router.push("login")
      }
      
    }
  } catch (error: any) {
    toast.error(error)
  }
})


const logout = async () => {
  await set_value("access_token", null)
  await set_value("refresh_token", null)
  toast.success("成功退出账号")
  router.push('login')
}

</script>

<template>
  <div class="navbar">
    <div class="navbar-start">
      <a class="btn btn-ghost text-xl avatar" @click="onClickGithub">Github</a>
    </div>
    
    <div class="navbar-center"> 
      <ul tabindex="0" class="menu menu-sm menu-horizontal mt-3 z-[1] p-2 rounded-box w-52">
        <li class="group">
          <RouterLink to="/show"  class="block p-2" :class="{ 'active': $route.path === '/show' }">演出</RouterLink>
        </li>

        <li class="group">
          <RouterLink to="/task"  class="block p-2" :class="{ 'active': $route.path === '/task' }">任务</RouterLink>
        </li>
        
        <li class="group">
          <RouterLink to="/order" class="block p-2" :class="{ 'active': $route.path === '/order' }">订单</RouterLink>
        </li>
      </ul>
    </div>
    <div class="navbar-end">
      <div class="dropdown dropdown-end">
        <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
          <div class="w-10 rounded-full">
            <img alt="Tailwind CSS Navbar component" :src="avatarSrc" />
          </div>
          <div>
            <label>{{ nickname }}</label>
          </div>
        </div>
        <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52">
          <li @click="logout"><a class="text-center">退出</a></li>
        </ul>
      </div>
    </div>
  </div>

  <div class="container mx-auto">
    <router-view v-slot="{ Component }">
      <keep-alive>
        <component :is="Component" :key="$route.name" v-if="$route.meta.keepAlive" />
      </keep-alive>
      <component :is="Component" :key="$route.name" v-if="!$route.meta.keepAlive" />
    </router-view>
  </div>
</template>


<style scoped>


</style>
