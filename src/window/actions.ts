import { WebviewWindow } from '@tauri-apps/api/window'
import { emit } from '@tauri-apps/api/event'
import EventName from '../constant/event'

/**
 * @desc 创建新窗口
 */
export async function createWin(args: any) {
    await emit(EventName.WIN_CREATE, args)
}

/**
 * @desc 获取窗口
 * @param args {string}
 */
export async function getWin(label: string) {
    return await WebviewWindow.getByLabel(label)
}

/**
 * @desc 设置窗口
 * @param type {string} 'show'|'hide'|'close'|'min'|'max'|'max2min'|'exit'|'relaunch'
 */
export async function setWin(type: string) {
    await emit('win-' + type)
}


