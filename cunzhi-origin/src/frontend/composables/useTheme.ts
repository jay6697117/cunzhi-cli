import { invoke } from '@tauri-apps/api/core'
import { computed, ref } from 'vue'
import { applyThemeVariables, getTheme } from '../theme'

export function useTheme() {
  // 先尝试从localStorage获取主题，避免初始化闪烁
  const savedTheme = localStorage.getItem('app-theme') || 'dark'
  const currentTheme = ref(savedTheme)

  // 立即应用保存的主题
  if (savedTheme) {
    applyThemeVariables(savedTheme)
  }

  // 计算 Naive UI 主题
  const naiveTheme = computed(() => {
    return getTheme(currentTheme.value)
  })

  // 应用主题
  function applyTheme(theme: string) {
    // 应用主题变量和类
    applyThemeVariables(theme)
    currentTheme.value = theme

    // 保存到localStorage，确保下次启动时能立即应用
    localStorage.setItem('app-theme', theme)
  }

  // 切换主题
  async function setTheme(theme: string) {
    try {
      // 先应用前端主题，确保用户立即看到变化
      applyTheme(theme)
      // 然后保存到后端
      await invoke('set_theme', { theme })
    }
    catch (error) {
      console.error('保存主题设置失败:', error)
      // 即使保存失败，前端主题已经应用，用户体验不受影响
    }
  }

  // 加载主题设置
  async function loadTheme() {
    try {
      const theme = await invoke('get_theme')
      // 确保主题值有效
      const validTheme = (theme === 'light' || theme === 'dark') ? theme : 'dark'

      // 只有当后端主题与当前主题不同时才应用
      if (validTheme !== currentTheme.value) {
        applyTheme(validTheme as string)
      }
    }
    catch (error) {
      console.error('加载主题失败:', error)
      // 如果没有localStorage缓存，则使用默认深色主题
      if (!localStorage.getItem('app-theme')) {
        applyTheme('dark')
      }
    }
  }

  return {
    currentTheme,
    naiveTheme,
    setTheme,
    loadTheme,
  }
}
