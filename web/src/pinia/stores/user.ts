import { defineStore } from 'pinia'
import { ref } from 'vue'

// 用户信息接口
export interface UserInfo {
  username: string
  token: string
  // 可以根据需要添加其他用户信息字段
}

// 创建用户 store
export const useUserStore = defineStore('user', () => {
  // 用户信息状态
  const userInfo = ref<UserInfo | null>(null)
  const isLoggedIn = ref(false)
  
  // 设置用户信息
  const setUserInfo = (info: UserInfo) => {
    userInfo.value = info
    isLoggedIn.value = true
    
    // 将用户信息存储到本地存储
    localStorage.setItem('userInfo', JSON.stringify(info))
  }
  
  // 清除用户信息
  const clearUserInfo = () => {
    userInfo.value = null
    isLoggedIn.value = false
    
    // 从本地存储清除用户信息
    localStorage.removeItem('userInfo')
  }
  
  // 初始化用户信息（从本地存储恢复）
  const initUserInfo = () => {
    const storedUserInfo = localStorage.getItem('userInfo')
    if (storedUserInfo) {
      try {
        const info = JSON.parse(storedUserInfo)
        userInfo.value = info
        isLoggedIn.value = true
      } catch (error) {
        console.error('Failed to parse stored user info:', error)
        localStorage.removeItem('userInfo')
      }
    }
  }
  
  return {
    userInfo,
    isLoggedIn,
    setUserInfo,
    clearUserInfo,
    initUserInfo
  }
})