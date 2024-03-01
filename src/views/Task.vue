<template>
    <div class="flex">
        <!-- 左边部分 -->
        <div class="flex-1 p-4 bg-gray-100 flex items-center justify-center">
            <div class="w-3/4">
                <h2 class="text-lg font-semibold mb-4">购票信息</h2>
                <div class="mb-4">
                    <label class="block font-semibold text-gray-800 mb-2">演唱会:</label>
                    <p>{{ show.showName }}</p>
                </div>
                
                <div class="mb-4">
                    <label class="block font-semibold text-gray-800 mb-2">场次:</label>
                    <p>{{ session.sessionName }}</p>
                </div>
                <div class="mb-4">
                    <label class="block font-semibold text-gray-800 mb-2">开售时间:</label>
                    <p v-if="session.sessionStatus == 'ON_SALE'">已开售</p>
                    <p v-else>{{ sessionDateString }}</p>
                </div>
                <div class="mb-4">
                    <label class="block font-semibold text-gray-800 mb-2">票档:</label>
                    <select v-model="selectedSeatPlanIndex" class="select select-success w-full max-w-xs">
                        <option v-for="(item, index) in seatPlans" :key="item.seatPlanId" :value="index">
                            {{ item.seatPlanName }} (可购:{{ item.canBuyCount }})
                        </option>
                    </select>
                    <button @click="refreshPage" class="btn btn-primary px-6 " :disabled="isRunning">刷新库存</button>
                </div>
                <!-- <div class="mb-4">
                    <label class="block font-semibold text-gray-800 mb-2">收货地址:</label>
                    <select v-model="address" class="select select-success w-full max-w-xs">
                        <option value="address1">地址1</option>
                        <option value="address2">地址2</option>
                        <option value="address3">地址3</option>
                    </select>
                </div> -->
                <div class="mb-4 flex items-center">
                    <label class="block font-semibold text-gray-800 mr-2">购票数量:</label>
                    <div class="flex items-center">
                        <button @click="decrementQuantity" class="px-3 py-1 bg-blue-500 text-white rounded-l-md">-</button>
                        <input type="text" v-model="quantity" class="w-10 text-center border border-gray-300" readonly>
                        <button @click="incrementQuantity" class="px-3 py-1 bg-blue-500 text-white rounded-r-md">+</button>
                    </div>
                </div>
                <div class="mb-4">
                    <label class="block font-semibold text-gray-800 mb-2">观演人:</label>
                    <label v-for="(item in audiences" :key="item.id" class="label cursor-pointer">
                        <span class="label-text">{{ item.name }}</span>
                        <span class="label-text">{{ item.description }}</span>
                        <span class="label-text">{{ item.idNo }}</span>
                        <input type="checkbox" class="checkbox" :value="item.id" v-model="selectedAudienceIdList" />
                    </label>
                </div>
                <div class="mb-4 flex">
                    <button @click="submit" class="btn btn-outline btn-primary px-6 " :disabled="isRunning">运行</button>

                    <button @click="stop" class="btn btn-outline btn-accent px-6 ml-6 bg-gray-100 text-white"
                        :disabled="!isRunning">停止</button>
                </div>
            </div>
        </div>

        <!-- 右边部分 -->
        <div class="flex-1 p-4 bg-gray-100 flex items-center justify-center">
            <div class="w-3/4">
                <h2 class="text-lg font-semibold mb-4">日志</h2>
                <div class="mb-4">
                    <textarea class="log_textarea w-full border border-gray-300 text-black max-auto h-max"
                        v-model="logData" readonly></textarea>
                </div>
            </div>
        </div>
    </div>
</template>
  
<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue';
import { get_value } from '../store';
import { invoke } from '@tauri-apps/api';
import { SeatPlan, Audiences, CurShowData, GetUserAudiencesResult, GetSeatPlansResult, Concert, Session } from '../types';
import { toast } from 'vue3-toastify';
import router from '../router';
import { UnlistenFn, emit, listen } from '@tauri-apps/api/event';
import AppEvent from '../constant/event'

const selectedAudienceIdList = ref([] as string[]);
const show = ref([] as unknown as Concert)
const session = ref([] as unknown as Session)
const seatPlans = ref([] as SeatPlan[]);
const selectedSeatPlanIndex = ref(0)
const audiences = ref([] as Audiences[]);
const quantity = ref(1);
const unlisten = ref(undefined as unknown as UnlistenFn);
const logData = ref("")
const isRunning = ref(false);
const incrementQuantity = () => { quantity.value++; };
const decrementQuantity = () => { if (quantity.value > 1) quantity.value--; };

const sessionDateString = computed(() => {
      const date = new Date(session.value.sessionSaleTime as number);
      return date.toLocaleString(); // 使用 toLocaleString() 方法转换为日期字符串
});

const selectedAudiences = (): Audiences[] => {
    const items: Audiences[] = [];
    const audiencesArray = audiences.value || []; // 确保audiences存在且是一个数组
    const selectedIdsArray = selectedAudienceIdList.value || []; // 确保selectedAudienceIdList存在且是一个数组
    
    audiencesArray.forEach(element => {
        if (element.id && selectedIdsArray.includes(element.id)) {
            items.push(element);
        }
    });

    return items;
};

const submit = async () => {
    console.log("开始运行")
    isRunning.value = true
    
    if (quantity.value != selectedAudienceIdList.value.length) {
        toast.error("您选择的购票数量和观演人数量不一致, 请检查!")
        isRunning.value = false
        return
    }
    toast.success("提交成功, 开始购票流程!")
    console.log(selectedAudiences())

    try {
       await invoke('buy_tickets', {
        show: show.value,
        session: session.value,
        seatPlan: seatPlans.value[selectedSeatPlanIndex.value],
        ticketNum: quantity.value,
        audiences: selectedAudiences()
    })
    } catch (error: any) {
        toast.error(error)
    }finally{
        isRunning.value = false
    }
  
    
};

const stop = async () => {
    await emit("stop-buy-tickets", {})
    isRunning.value = false
}

const refreshPage = async () => {
    const cur_show_data = (await get_value('cur_show_data')) as CurShowData
    console.log(cur_show_data)
    show.value = cur_show_data.show
    session.value = cur_show_data.session as Session
    seatPlans.value = cur_show_data.seatPlans
    selectedSeatPlanIndex.value = cur_show_data.seatPlanIndex

    const get_seat_plans_res: GetSeatPlansResult = await invoke('get_seat_plans', { showId: show.value.showId, sessionId: session.value.bizShowSessionId })
    if (get_seat_plans_res.statusCode == 200) {
        seatPlans.value = get_seat_plans_res.data.seatPlans
    }

    const res: GetUserAudiencesResult = await invoke('get_user_audiences')
    if (res.statusCode == 200) {
        audiences.value = res.data
    } else {
        toast.error("当前账号未添加观演人信息, 请在APP添加观演人后再进行购票!")
        return
    }
}

// 重新选择演唱会后需要刷新内容
watch(async() => router.currentRoute.value.fullPath, async () => {
    if (router.currentRoute.value.query?.refresh) {
        await refreshPage()
    }
});

onMounted(async () => {
    await refreshPage();
    unlisten.value = await listen(AppEvent.SHOW_LOG, (event: any) => {

        logData.value = event.payload.msg + '\n' + logData.value; // 将新的日志插入到日志数据的开头
    })
})

</script>



  
<style>
.log_textarea {
    min-height: 500px;
}
</style>
  