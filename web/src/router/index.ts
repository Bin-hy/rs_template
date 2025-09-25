import { createRouter, createWebHashHistory, type RouteRecordRaw } from 'vue-router'
import { routerConfig } from './config'
import { flatMultiLevelRoutes } from './helper'
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
    }
]

export const router = createRouter({
    history: routerConfig.history,
    routes: routerConfig.thirdLevelRouteCache ? flatMultiLevelRoutes(constantRoutes) : constantRoutes
})
