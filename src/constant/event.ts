
import { TauriEvent } from "@tauri-apps/api/event";

enum AppEvent {
    CREATED_SUCCESS_SUFFIX = '://created',
    CREATED_ERROR_SUFFIX = '://error',

    TAURI_CREADED = 'tauri' + CREATED_SUCCESS_SUFFIX,
    TAURI_ERROR = 'tauri' + CREATED_ERROR_SUFFIX,

    WIN_CREATE = 'win://create',   // 创建新创建
    WIN_SHOW = 'win://show',       // 显示窗体
    WIN_HIDE = 'win://hide',       // 隐藏窗体
    WIN_EXIT = 'win://exit',       // 退出窗体
    WIN_RELAUNCH = 'win://relaunch',  // 重启应用


    LOGIN_SUCCESS = 'login-success', // 登录成功
    LOGIN_FAIL = 'login-fail', // 登录失败

    SHOW_LOG = 'show-log'
}

const EventName = {
    ...AppEvent,
    ...TauriEvent
}

export default EventName
