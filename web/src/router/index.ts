import { createRouter, createWebHashHistory, type RouteRecordRaw } from 'vue-router'
import { routerConfig } from './config'
import { flatMultiLevelRoutes } from './helper'
import { useUserStore } from '@/pinia/stores/user'
/**
 * @name 常驻路由
 * @description 除了 redirect/403/404/login 等隐藏页面，其他页面建议设置唯一的 Name 属性
 */
export const constantRoutes: RouteRecordRaw[] = [
    {
        "path": "/redirect",
        "component": import("@/pages/redirect/index.vue")
    },
    {
        "path": "/403",
        "name": "403",
        "component": import("@/pages/errors/403.vue"),
    },
    {
        "path": "/404",
        "name": "404",
        "component": import("@/pages/errors/404.vue"),
    },
    {
        "path": "/",
        "name": "login",
        "component": import("@/pages/login/index.vue"),
        "meta": {
            hidden: true // 隐藏在导航菜单中
        }
    },
    {
        "path": "/dashboard",
        "name": "dashboard",
        "component": import("@/pages/dashboard/index.vue"),
        "meta": {
            requiresAuth: true, // 需要登录才能访问
            title: '仪表盘'
        }
    }
]

export const router = createRouter({
    history: routerConfig.history,
    routes: routerConfig.thirdLevelRouteCache ? flatMultiLevelRoutes(constantRoutes) : constantRoutes
})

// 添加导航守卫
router.beforeEach((to, from, next) => {
    // 如果路由需要认证
    if (to.meta.requiresAuth) {
        const userStore = useUserStore()
        // 检查用户是否已登录
        if (userStore.isLoggedIn) {
            next() // 已登录，继续访问
        } else {
            next({ name: 'login' }) // 未登录，重定向到登录页
        }
    } else {
        next() // 不需要认证，继续访问
    }
})

// 处理404路由
router.beforeEach((to, from, next) => {
    if (to.matched.length === 0) {
        next({ name: '404' })
    } else {
        next()
    }
})
