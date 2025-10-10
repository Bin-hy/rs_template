<script lang="ts" setup>
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/pinia/stores/user'

// 表单数据
const form = reactive({
  username: '',
  password: '',
  remember: false
})

// 表单验证错误信息
const errors = reactive({
  username: '',
  password: ''
})

// 登录状态
const loading = ref(false)
const router = useRouter()
const userStore = useUserStore()

// 表单验证
const validateForm = (): boolean => {
  let isValid = true

  // 重置错误信息
  errors.username = ''
  errors.password = ''

  // 验证用户名
  if (!form.username.trim()) {
    errors.username = '请输入用户名'
    isValid = false
  }

  // 验证密码
  if (!form.password) {
    errors.password = '请输入密码'
    isValid = false
  } else if (form.password.length < 6) {
    errors.password = '密码长度不能少于6位'
    isValid = false
  }

  return isValid
}

// 登录处理
const handleLogin = async () => {
  // 验证表单
  if (!validateForm()) {
    return
  }

  try {
    loading.value = true

    // 模拟登录请求
    await new Promise(resolve => setTimeout(resolve, 1000))

    // 假设登录成功，存储用户信息
    userStore.setUserInfo({
      username: form.username,
      token: 'mock-token-123456'
    })

    // 如果勾选了记住我，可以在这里存储到本地存储
    if (form.remember) {
      localStorage.setItem('rememberedUser', form.username)
    } else {
      localStorage.removeItem('rememberedUser')
    }

    // 登录成功后跳转到首页或其他页面
    router.push('/dashboard') // 这里假设首页路径是/dashboard，根据实际情况修改

  } catch (error) {
    console.error('登录失败:', error)
    alert('登录失败，请检查用户名和密码')
  } finally {
    loading.value = false
  }
}

// 检查是否有记住的用户
const rememberedUser = localStorage.getItem('rememberedUser')
if (rememberedUser) {
  form.username = rememberedUser
  form.remember = true
}
</script>

<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center p-4">
    <div class="w-full max-w-md bg-white rounded-xl shadow-2xl overflow-hidden">
      <!-- 登录表单头部 -->
      <div class="bg-gradient-to-r from-blue-600 to-indigo-600 p-6 text-white">
        <h1 class="text-2xl font-bold text-center">账号登录</h1>
        <p class="text-blue-100 text-center mt-1">欢迎回来，请登录您的账号</p>
      </div>

      <!-- 登录表单内容 -->
      <div class="p-6">
        <form @submit.prevent="handleLogin" class="space-y-4">
          <!-- 用户名输入框 -->
          <div>
            <label for="username" class="block text-sm font-medium text-gray-700 mb-1">
              用户名
            </label>
            <div class="relative">
              <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                </svg>
              </div>
              <input
                id="username"
                v-model="form.username"
                type="text"
                class="w-full pl-10 pr-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition duration-200 ease-in-out"
                placeholder="请输入用户名"
                :class="{ 'border-red-500': errors.username }"
                @blur="validateForm"
              />
            </div>
            <p v-if="errors.username" class="text-red-500 text-xs mt-1">{{ errors.username }}</p>
          </div>

          <!-- 密码输入框 -->
          <div>
            <label for="password" class="block text-sm font-medium text-gray-700 mb-1">
              密码
            </label>
            <div class="relative">
              <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
                </svg>
              </div>
              <input
                id="password"
                v-model="form.password"
                type="password"
                class="w-full pl-10 pr-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition duration-200 ease-in-out"
                placeholder="请输入密码"
                :class="{ 'border-red-500': errors.password }"
                @blur="validateForm"
              />
            </div>
            <p v-if="errors.password" class="text-red-500 text-xs mt-1">{{ errors.password }}</p>
          </div>

          <!-- 记住我和忘记密码 -->
          <div class="flex items-center justify-between">
            <div class="flex items-center">
              <input
                id="remember"
                v-model="form.remember"
                type="checkbox"
                class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
              />
              <label for="remember" class="ml-2 block text-sm text-gray-700">
                记住我
              </label>
            </div>
            <a href="#" class="text-sm font-medium text-blue-600 hover:text-blue-500 transition-colors">
              忘记密码？
            </a>
          </div>

          <!-- 登录按钮 -->
          <button
            type="submit"
            class="w-full flex justify-center py-2 px-4 border border-transparent rounded-lg shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition duration-200 ease-in-out"
            :disabled="loading"
          >
            <span v-if="loading" class="mr-2 h-4 w-4 border-2 border-white border-t-transparent rounded-full animate-spin"></span>
            登录
          </button>
        </form>

        <!-- 注册账号提示 -->
        <div class="mt-6 text-center">
          <p class="text-sm text-gray-600">
            还没有账号？
            <a href="#" class="font-medium text-blue-600 hover:text-blue-500 transition-colors">
              立即注册
            </a>
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
