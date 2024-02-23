import { WebviewWindow, appWindow, getAll } from '@tauri-apps/api/window'
import { relaunch, exit } from '@tauri-apps/api/process'
import { Event, emit, listen } from '@tauri-apps/api/event'
import { setWin } from './actions'
import EventName from '../constant/event'

// 系统参数配置
export const windowConfig = {
    label: null,            // 窗口唯一label
    title: '',              // 窗口标题
    url: '',                // 路由地址url
    width: 400,             // 窗口宽度
    height: 400,            // 窗口高度
    minWidth: null,         // 窗口最小宽度
    minHeight: null,        // 窗口最小高度
    x: null,                // 窗口相对于屏幕左侧坐标
    y: null,                // 窗口相对于屏幕顶端坐标
    center: true,           // 窗口居中显示
    resizable: true,        // 是否支持缩放
    maximized: false,       // 最大化窗口
    decorations: true,     // 窗口是否有边框及导航条
    alwaysOnTop: true,     // 置顶窗口
}


class Windows {
    
    mainWin = null

    constructor() {
        this.mainWin = null
    }

    // 获取窗口
    getWin(label: string) {
        return WebviewWindow.getByLabel(label)
    }

    // 获取全部窗口
    getAllWin() {
        return getAll()
    }

    // 创建新窗口
    async createWin(options: any) {
        const args = Object.assign({}, windowConfig, options)

        // 判断窗口是否存在
        const existWin = getAll().find(w => w.label == args.label)
        if(existWin) {
            if(existWin.label.indexOf('main') == -1) {
                await existWin?.unminimize()
                await existWin?.setFocus()
                return
            }
            await existWin?.close()
        }

        // 创建窗口对象
        let win = new WebviewWindow(args.label, args)
        
        // 是否最大化
        if(args.maximized && args.resizable) {
            win.maximize()
        }

        // 窗口创建完毕
        win.once(EventName.TAURI_CREADED, async(event: Event<any>) => {
            await emit( event.windowLabel + EventName.CREATED_SUCCESS_SUFFIX, event.payload)
        })

        // 窗口创建错误
        win.once(EventName.TAURI_ERROR, async(event: Event<any>) => {
            await emit(event.windowLabel + EventName.CREATED_ERROR_SUFFIX, event.payload)
        })
    }

    // 开启主进程监听事件
    async listen() {
        // 创建新窗体
        await listen(EventName.WIN_CREATE, (event: Event<any>) => {
            this.createWin(event.payload)
        })

        // 显示窗体
        await listen(EventName.WIN_SHOW, async(event: Event<any>) => {
            console.log(event)
            if(appWindow.label.indexOf('main') == -1) return
            await appWindow.show()
            await appWindow.unminimize()
            await appWindow.setFocus()
        })

        // 隐藏窗体
        await listen(EventName.WIN_HIDE, async(event) => {
            console.log(event)
            if(appWindow.label.indexOf('main') == -1) return
            await appWindow.hide()
        })
        
        
        await listen(EventName.WIN_EXIT, async(event) => {
            console.log(event)
            setWin('logout')
            await exit()
        })

        // 重启应用
        await listen(EventName.WIN_RELAUNCH, async(event) => {
            console.log(event)
            await relaunch()
        })
    }
}

export default Windows