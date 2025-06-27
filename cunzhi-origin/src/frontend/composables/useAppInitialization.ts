import { ref } from 'vue'
import { initMcpTools } from './useMcpTools'
import { useSettings } from './useSettings'
import { useTheme } from './useTheme'
import { useVersionCheck } from './useVersionCheck'

/**
 * 应用初始化组合式函数
 */
export function useAppInitialization(mcpHandler: ReturnType<typeof import('./useMcpHandler').useMcpHandler>) {
  const isInitializing = ref(true)
  const { loadTheme } = useTheme()
  const settings = useSettings()
  const { autoCheckUpdate } = useVersionCheck()
  const { checkMcpMode, setupMcpEventListener } = mcpHandler

  /**
   * 初始化应用
   */
  async function initializeApp() {
    try {
      // 加载主题设置
      await loadTheme()

      // 检查是否为MCP模式
      const { isMcp, mcpContent } = await checkMcpMode()

      // 无论是否为MCP模式，都加载窗口设置
      await settings.loadWindowSettings()
      await settings.loadWindowConfig()

      // 初始化MCP工具配置（在非MCP模式下）
      if (!isMcp) {
        await initMcpTools()
        await setupMcpEventListener()
      }

      // 自动检查版本更新并弹窗（非阻塞）
      autoCheckUpdate().catch(() => {
        // 静默处理版本检查失败
      })

      // 结束初始化状态
      isInitializing.value = false

      return { isMcp, mcpContent }
    }
    catch (error) {
      console.error('应用初始化失败:', error)
      isInitializing.value = false
      throw error
    }
  }

  return {
    isInitializing,
    initializeApp,
  }
}
