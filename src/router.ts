import { createRouter, createWebHashHistory, RouterOptions, Router, RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
    {
        path: '/',
        name: 'Home',
        component: () => import("./views/Home.vue"),
        meta: {
            keepAlive: true,
        },
        children: [
            { 
                path: '/show', 
                name: 'Show', 
                component: () => import("./views/Show.vue"),
                meta: {
                    keepAlive: true,
                }
            },
            { 
                path: '/task/:refresh?', 

                name: 'Task', 
                component: () => import("./views/Task.vue"),
                meta: {
                    keepAlive: true,
                }
            },
            { 
                path: '/order', 
                name: 'Order', 
                component: () => import("./views/Order.vue"),
                meta: {
                    keepAlive: true,
                }
            },
        ]
    },    
    {
        path: "/login",
        name: "Login",
        component: () => import("./views/Login.vue"),
    }
]

// RouterOptions是路由选项类型
const options: RouterOptions = {
 history: createWebHashHistory(),
 routes,
}

// Router是路由对象类型
const router: Router = createRouter(options)

export default router