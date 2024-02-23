<template>
  <div class="fixed inset-0 overflow-y-auto flex justify-center items-center bg-gray-500 bg-opacity-50">
    <div class="bg-white rounded-md p-8 w-96 border border-base-300">
      <!-- <div class="flex justify-end">
        <button @click="closeLogin" class="text-gray-400 hover:text-gray-600">
          <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </button>
      </div> -->
      
      <div>
        <h2 class="text-3xl font-extrabold text-gray-900 text-center mb-6">短信登录</h2>
        <form @submit.prevent="login">
          <div>
            <label for="phone-number" class="sr-only">Phone Number</label>
            <input id="phone-number" name="phone-number" type="tel" autocomplete="tel" required v-model="phoneNumber"
              class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
              placeholder="手机号码">
          </div>

          <div class="flex items-center space-x-4 mt-4" v-if="imageVerificationSrc">
            <input id="image-verification-code" name="image-verification-code" type="text"
              autocomplete="image-verification-code" required v-model="imageVerificationCode"
              class="appearance-none rounded-none relative block w-1/2 px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
              placeholder="请输入验证码">
            <img v-if="imageVerificationSrc" :src="imageVerificationSrc" alt="Image Verification Code"
              @click="generatePhotoCode" class="appearance-none rounded-none relative block w-1/2 px-3 py-2 h-14">
          </div>
          <div class="flex items-center space-x-4 mt-4">
            <input id="verification-code" name="verification-code" type="text" autocomplete="verification-code" required
              v-model="verificationCode"
              class="appearance-none rounded-none relative block w-1/2 px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
              placeholder="验证码">
            <button type="button" @click="sendVerificationCode" :disabled="!phoneNumber || phoneNumber.length !== 11"
              class="w-1/2 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-green-500 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
              发送验证码
            </button>
          </div>

          <div class="mt-6">
            <button type="submit"
              class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
              登录
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>


<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { ref } from 'vue'
import { set_value } from '../store';
import router from '../router';

import { SendVerificationCodeResult, GeneratePhotoCodeResult, LoginResult } from '../types'
import { toast } from 'vue3-toastify';

const phoneNumber = ref('');
const verificationCode = ref('');
const imageVerificationCode = ref('');
const imageVerificationSrc = ref('');

// const imageVerificationSrc = ref("");
const sendVerificationCode = async () => {
  try {
    const res: SendVerificationCodeResult = await invoke("send_verification_code", { "mobile": phoneNumber.value, token: imageVerificationCode.value });
    if (res.data == true) {
      toast.success("短信验证码发送成功!")
      return
    }

    if (res.statusCode == 15012012) {
      toast.error("验证码发送失败, 请输入图形验证码!")
      await generatePhotoCode();
      return
    }
    toast.error(res.comments);
  } catch (error: any) {
    toast.error(error)
  }
}

const generatePhotoCode = async () => {
  try {
    const res: GeneratePhotoCodeResult = await invoke('generate_photo_code', { 'mobile': phoneNumber.value })
    if (res.statusCode == 200) {
      imageVerificationSrc.value = res.data.baseCode;
      return
    }
    toast.error(res.comments)
  } catch (error: any) {
    toast.error(error)
  }
}

const login = async () => {
  try {
    const res: LoginResult = await invoke('login_by_mobile', { mobile: phoneNumber.value, smsCode: verificationCode.value });
    if (res.statusCode == 200) {
      await set_value("access_token", res.data.accessToken)
      await set_value("refresh_token", res.data.refreshToken)
      toast.success("登录成功")
      router.push('show')
    } else {
      toast.error(`登录失败, ${res.comments}`)
    }
  } catch (error: any) {
    toast.error(error)
  }


}



</script>
