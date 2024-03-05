<template>
    <div class="flex">
        <!-- 左边部分 -->
        <div class="flex-1 p-4 bg-gray-100 flex items-center justify-center">
            <div class="w-3/4">
                <h2 class="text-lg font-semibold mb-4 items-center justify-center">购票信息</h2>
                <div class="mb-4 flex items-center">
                    <img :src="show.posterUrl" alt="Show Image" class="w-32 h-32 object-cover mb-2 mr-4">
                    <p class="block font-semibold text-gray-800 mb-2">{{ show.showName }}</p>
                </div>

                <div class="mb-4 flex items-center">
                    <label v-for="item in noteInfos" class="block font-semibold text-gray-800 mb-2 mr-4">
                        <input type="checkbox" v-model="item.type" disabled readonly /> {{ item.name }}
                    </label>
                </div>

                <div class="mb-4 flex items-center">
                    <label class="block font-semibold text-gray-800 mb-2 mr-4">类型:</label>
                    <p class="block font-semibold text-gray-800 mb-2">{{ show?.backendCategory?.displayName }}</p>
                </div>

                <div class="mb-4 flex items-center">
                    <label class="block font-semibold text-gray-800 mb-2 mr-4">场次:</label>
                    <p class="block font-semibold text-gray-800 mb-2">{{ session.sessionName }}</p>
                </div>
                <div class="mb-4 flex items-center">
                    <label class="block font-semibold text-gray-800 mb-2 mr-4">开售时间:</label>
                    <p class="block font-semibold text-gray-800 mb-2" v-if="session.sessionStatus == 'ON_SALE'">已开售</p>
                    <p class="block font-semibold text-gray-800 mb-2" v-else>{{ sessionDateString }}</p>
                </div>
                <div class="mb-4 flex items-center">
                    <label class="block font-semibold text-gray-800 mb-2 mr-4">票档:</label>
                    <select v-model="selectedSeatPlanIndex" class="select select-success w-full max-w-xs mr-4">
                        <option v-for="(item, index) in seatPlans" :key="item.seatPlanId" :value="index">
                            {{ item.seatPlanName }} (可购:{{ item.canBuyCount }})
                        </option>
                    </select>
                    <button @click="refreshPage" class="btn btn-primary px-6 " :disabled="isRunning">刷新库存</button>
                </div>
                <div v-if="isExpress" class="mb-4" >
                    <label class="block font-semibold text-gray-800 mb-2">收货地址:</label>
                    <select v-model="selectedAddressIndex" class="select select-success w-full max-w-xs">
                        <option v-for="(item, index) in address" :key="item.addressId" :value="index">
                          {{ item.username }}  {{ item.cellphone }} {{item.location.province }} {{item.location.city}} {{item.location.district }} {{ item.detailAddress }}
                        </option>
                    </select>
                </div>
                <div class="mb-4 flex items-center">
                    <label class="block font-semibold text-gray-800 mb-2 mr-4 mr-2">购票数量:</label>
                    <div class="flex items-center">
                        <button @click="decrementQuantity" class="px-3 py-1 bg-blue-500 text-white rounded-l-md">-</button>
                        <input type="text" v-model="quantity" class="w-10 text-center border border-gray-300" readonly>
                        <button @click="incrementQuantity" class="px-3 py-1 bg-blue-500 text-white rounded-r-md">+</button>
                    </div>
                </div>
                <div class="mb-4 items-center">
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
                    <textarea class="log_textarea w-full border border-gray-300 text-black max-auto h-max" v-model="logData"
                        readonly></textarea>
                </div>
            </div>
        </div>
    </div>
</template>
  
<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue';
import { get_value } from '../store';
import { invoke } from '@tauri-apps/api';
import { SeatPlan, Audiences, CurShowData, GetUserAudiencesResult, GetSeatPlansResult, Show, Session, BuyTicketParam, GetShowDetailResult, NoteInfo, GetAddressResult, Address } from '../types';
import { toast } from 'vue3-toastify';
import router from '../router';
import { UnlistenFn, emit, listen } from '@tauri-apps/api/event';
import AppEvent from '../constant/event'

const selectedAddressIndex = ref(0)
const selectedAudienceIdList = ref([] as string[])
const show = ref([] as unknown as Show)
const session = ref([] as unknown as Session)
const seatPlans = ref([] as SeatPlan[])
const selectedSeatPlanIndex = ref(0)
const address = ref([] as Address[])
const audiences = ref([] as Audiences[])
const quantity = ref(1)
const noteInfos = ref([] as NoteInfo[])
const isRealname = ref(false)
const isExpress = ref(false)
const unlisten = ref(undefined as unknown as UnlistenFn)
const logData = ref("")
const isRunning = ref(false);
const incrementQuantity = () => { if (quantity.value < 4) quantity.value++; };
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
    if (!['YanChangHui', 'Livehouse'].includes(show.value.backendCategory.name)) {
        toast.success(`演出类型:${show.value.backendCategory.displayName}暂不支持!`);
        return
    }
    isRunning.value = true

    if (quantity.value != selectedAudienceIdList.value.length) {
        toast.error("您选择的购票数量和观演人数量不一致, 请检查!")
        isRunning.value = false
        return
    }
    toast.success("提交成功, 开始购票流程!")
    console.log(selectedAudiences())

    try {
        const params: BuyTicketParam = {
            show: show.value,
            session: session.value,
            seatPlan: seatPlans.value[selectedSeatPlanIndex.value],
            ticketNum: quantity.value,
            audiences: selectedAudiences(),
            isExpress: isExpress.value,
            isRealname: isRealname.value,
            address: address.value[selectedAddressIndex.value]
        }
        await invoke('buy_tickets', { params })
    } catch (error: any) {
        toast.error(error)
    } finally {
        isRunning.value = false
    }


};

const stop = async () => {
    await emit("stop-buy-tickets", {})
    isRunning.value = false
}

const refreshPage = async () => {
    const cur_show_data = (await get_value('cur_show_data')) as CurShowData
    show.value = cur_show_data.show
    const result: GetShowDetailResult = await invoke('get_show_detail', { showId: show.value.showId })
    if (result.statusCode == 200) {
        noteInfos.value = []
        result.data.noteInfos.forEach( element => {
            if (element.code == 'EXPRESS') {
                isExpress.value = element.type
                noteInfos.value.push(element)
            }
            else if (element.code == 'REAL_NAME_TICKET') {
                isRealname.value = element.type
                noteInfos.value.push(element)
            }else if (element.code == 'E_TICKET') {
                noteInfos.value.push(element)
            }
        });
    }
    session.value = cur_show_data.session
    seatPlans.value = cur_show_data.seatPlans
    selectedSeatPlanIndex.value = cur_show_data.seatPlanIndex
    const res2 = await invoke("get_seat_plans_stock", { showId: show.value.showId, sessionId: session.value.bizShowSessionId })
    console.log(res2)
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

    if (isExpress)  {
        const getAddressRes: GetAddressResult = await invoke('get_user_address')
       
        if (getAddressRes.statusCode == 200) {
            address.value = getAddressRes.data
        }
    }
}

// 重新选择演唱会后需要刷新内容
watch(async () => router.currentRoute.value.fullPath, async () => {
    if (router.currentRoute.value.query?.refresh) {
        await refreshPage()
    }
});

onMounted(async () => {
    
    unlisten.value = await listen(AppEvent.SHOW_LOG, (event: any) => {

        logData.value = event.payload.msg + '\n' + logData.value; // 将新的日志插入到日志数据的开头
    })

    await refreshPage();
})

</script>



  
<style>
.log_textarea {
    min-height: 500px;
}
</style>
  