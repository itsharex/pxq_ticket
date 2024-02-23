<template>
  <div class="p-4">
    <div class="flex items-center mb-4">
      <select v-model="sortBy" class="select select-bordered max-w-xs focus:outline-none">
        <option value="ATTENTION">热点排序</option>
        <option value="NEW">最新排序</option>
        <option value="RECOMMEND">推荐排序</option>
      </select>
      <input v-model="searchKeyword" type="text" class="flex-grow p-2 border border-gray-300 focus:outline-none"
        placeholder="搜索演唱会门票...">
      <button @click="search" class="p-2 bg-blue-500 text-white rounded-r-md">搜索</button>
    </div>


    <div v-if="showModal" class="fixed inset-0 flex items-center justify-center bg-gray-500 bg-opacity-50 ">
      <div class="bg-white p-6 rounded-md shadow-md">
        <h2 class="text-xl font-bold mb-4">{{ selectedConcert.showName }}</h2>

        <div class="p-4">
          <h4>场次:</h4>
          <select v-model="selectedSessionIndex" @change="updateSeatPlans()"
            class="select select-success w-full max-w-xs">
            <option v-for="(item, index) in sessions" :key="item.bizShowSessionId" :value="index">{{ item.sessionName }}
            </option>
          </select>
        </div>

        <div class="p-4">
          <h4>票档:</h4>
          <select v-model="selectedSeatPlanIndex" class="select select-success w-full max-w-xs">
            <option v-for="(item, index) in seatPlans" :key="item.seatPlanId" :value="index">{{ item.seatPlanName }}
            </option>
          </select>
        </div>

        <div class="mt-4 flex justify-end space-x-4 w-full max-w-xs">
          <div class="mt-4 flex justify-start space-x-4 w-full max-w-xs">
            <button @click="modalBtnFn" class="btn btn-primary rounded-md w-full max-w-xs">{{ modalBtnText }}</button>
          </div>

          <div class="mt-4 flex justify-start space-x-4 w-full max-w-xs">
            <button @click="showModal = false" class="btn btn-accent rounded-md w-full max-w-xs">关闭</button>
          </div>
        </div>

      </div>
    </div>


    <div v-if="loading" class="text-center">加载中...</div>
    <div v-else-if="error" class="text-center text-red-500">{{ error }}</div>

    <div v-else>
      <div v-if="concerts.length === 0 && !reachedEnd" class="text-center text-gray-500">暂无演唱会门票</div>
      <div v-else>
        <div v-for="(concert, index) in sortedConcerts" :key="index" class="border p-4 mb-4 flex items-start">
          <img :src="concert.posterUrl" alt="Concert Image" class="w-48 h-48 object-cover rounded-md mr-4">
          <div class="flex-grow">
            <h3 class="text-lg font-bold mb-2">{{ concert.showName }}</h3>
            <p><span class="font-bold">日期:</span> {{ concert.showDate }}</p>
            <p><span class="font-bold">城市:</span> {{ concert.cityName }}</p>
            <p><span class="font-bold">场地:</span> {{ concert.venueName }}</p>
            <p v-if="concert.latestSaleTime"><span class="font-bold">开售时间:</span> {{ concert.latestSaleTime }}</p>
            <div class="flex items-center mt-2">
              <p class="text-lg font-bold mr-auto">{{ concert.minOriginalPrice }} 元起</p>
              <div class="flex items-center mt-2">
                <button v-if="concert.showStatus === 'PENDING' && concert.latestSaleTime"
                  class="p-2 bg-blue-500 text-white rounded-md" @click="openModal(index, BtnType.WAIT)">加入抢票</button>
                <button v-else-if="concert.showStatus === 'PENDING'" class="p-2 bg-blue-500 text-white rounded-md"
                  @click="openModal(index, BtnType.REMIND)">添加提醒</button>
                <button v-else-if="concert.showStatus === 'ONSALE'" class="p-2 bg-blue-500 text-white rounded-md"
                  @click="openModal(index, BtnType.BUY)">立即购买</button>
                <button v-else-if="concert.showStatus === 'PRESALE'" class="p-2 bg-blue-500 text-white rounded-md"
                  @click="openModal(index, BtnType.SOLD_OUT)">缺票登记</button>
              </div>

            </div>
          </div>
        </div>

        <div v-if="loadingNextPage" class="text-center">加载中...</div>
        <div v-else-if="!reachedEnd" ref="scroll"></div>
        <div v-else class="text-center text-gray-500">已经没有更多内容了</div>
      </div>
    </div>
  </div>
</template>
  
<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { ref, computed, onMounted } from 'vue';
import { toast } from 'vue3-toastify';
import router from '../router';
import { set_value } from '../store';
import { Concert, CurShowData, QueryShowSessionResult, SeatPlan, Session } from '../types';



enum BtnType {
  REMIND = 1,  // 未公布开售时间
  WAIT,  // 已公布开售时间，但是未开始抢票
  BUY,  // 已开售并且有票
  SOLD_OUT // 已售完, 进行缺货登记
}
const sortBy = ref('NEW'); // 默认按热点排序
const searchKeyword = ref('');
const loading = ref(false);
const error = ref('');
const concerts = ref([] as Concert[]);
const currentPage = ref(1);
const loadingNextPage = ref(false);
const reachedEnd = ref(false);
const selectedConcert = ref({} as Concert);
const showModal = ref(false);
const sortedConcerts = computed(() => concerts.value);
const scroll = ref(null as unknown as Element);
const sessions = ref([] as Session[]);
const selectedSessionIndex = ref(0);
const seatPlans = ref([] as SeatPlan[]);
const selectedSeatPlanIndex = ref(0)
const modalBtnText = ref("提交")
const modalBtnType = ref()
const updateSeatPlans = async () => {
  seatPlans.value = sessions.value[selectedSessionIndex.value].seatPlans
  selectedSeatPlanIndex.value = 0
}
const openModal = async (index: number, btnType: BtnType) => {
  console.log('打开弹窗')
  showModal.value = true;
  selectedConcert.value = concerts.value[index];
  try {
    const show_sessions: QueryShowSessionResult = await invoke('query_show_sessions', { showId: selectedConcert.value.showId })
    if (show_sessions.statusCode == 200) {
      sessions.value = show_sessions.data
      selectedSessionIndex.value = 0;
      seatPlans.value = sessions.value[0].seatPlans
      selectedSeatPlanIndex.value = 0
    }
  } catch (error:any) {
    toast.error(error)
  }
  if (btnType == BtnType.REMIND) {
    modalBtnText.value = "抢票提醒"
    modalBtnType.value = BtnType.REMIND
  } else if (btnType == BtnType.BUY) {
    modalBtnText.value = "立即购买"
    modalBtnType.value = BtnType.BUY
  } else if (btnType == BtnType.WAIT) {
    modalBtnText.value = "加入抢票"
    modalBtnType.value = BtnType.WAIT
  } else {
    modalBtnText.value = "缺货登记"
    modalBtnType.value = BtnType.SOLD_OUT
  }
};


const search = async () => {
  currentPage.value = 1; // 重置页码
  loading.value = true;
  try {
    await fetchConcerts();
  } catch (error: any) {
    error.value = '加载失败，请重试';
  } finally {
    loading.value = false;
  }
};

const modalBtnFn = async () => {
  if (modalBtnType.value == BtnType.REMIND) {
    await addReminder()
  } else if (modalBtnType.value == BtnType.BUY) {
    await buyNow()
  } else if (modalBtnType.value == BtnType.WAIT) {
    await grabSale()
  } else {
    await ticketWaitlist()
  }
}

interface addReminderRes {
  statusCode: number,
  comments: string,
  data: {
    subscribed: boolean
  }
}
const addReminder = async () => {
  const res: addReminderRes = await invoke("add_reminder", { showId: selectedConcert.value.showId, sessionId: sessions.value[selectedSessionIndex.value].bizShowSessionId })
  if (res.statusCode == 200 && res.data.subscribed) {
    toast.success("预约抢票成功！票星球将在开售后第一时间通知您!")
  }
}

const buyNow = async () => {
  console.log("立即购买")
  goToTask()
}

const grabSale = async () => {
  console.log("抢购任务")
  goToTask()
}

const goToTask = async () => {
  const cur_show_data: CurShowData = {
    show: selectedConcert.value,
    session: sessions.value[selectedSessionIndex.value],
    seatPlans: seatPlans.value,
    seatPlanIndex: selectedSeatPlanIndex.value,
  }
  await set_value("cur_show_data", cur_show_data)

  router.push({
    path: '/task',
    query: { "refresh": "true"}
  })
}


interface ticketWaitlistRes {
  statusCode: number,
  comments: string,
  data: {
    subscribed: boolean
  }
}
const ticketWaitlist = async () => {
  const res: ticketWaitlistRes = await invoke("ticket_waitlist", {
    showId: selectedConcert.value.showId,
    sessionId: sessions.value[selectedSessionIndex.value].bizShowSessionId,
    seatPlanId: seatPlans.value[selectedSeatPlanIndex.value].seatPlanId
  })
  if (res.statusCode == 200 && res.data.subscribed) {
    toast.success("登记成功！票星球将在有票后第一时间通知您。!")
  }
}

const fetchConcerts = async () => {
  const response: any = await invoke("search_show_list", { keyword: searchKeyword.value, sortType: sortBy.value, page: currentPage.value });
  console.log(response);
  if (response.statusCode != 200) {
    return;
  }
  if (response.data.searchData.length == 0) {
    reachedEnd.value = true;
  } else {
    if (currentPage.value === 1) {
      concerts.value = response.data.searchData;
    } else {
      concerts.value = [...concerts.value, ...response.data.searchData];
    }
    currentPage.value++;
  }
};

const loadNextPage = async () => {
  if (!loadingNextPage.value && !reachedEnd.value) {
    loadingNextPage.value = true;
    try {
      await fetchConcerts();
    } catch (error: any) {
      console.error('加载下一页失败', error);
      toast.error(error);
    } finally {
      loadingNextPage.value = false;
    }
  }
};

const observeScroll = () => {
  const scrollObserver = new IntersectionObserver(entries => {
    const entry = entries[0];
    if (entry.isIntersecting && !loadingNextPage.value && !reachedEnd.value) {
      loadNextPage();
    }
  });
  scrollObserver.observe(scroll.value);
};


onMounted(async () => {
  await loadNextPage()
  observeScroll()
})


</script>
  
<style scoped>/* Add custom styles here */</style>
  