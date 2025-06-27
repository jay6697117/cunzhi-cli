<script setup lang="ts">
import { ref } from 'vue'
import IntroTab from '../tabs/IntroTab.vue'
import McpToolsTab from '../tabs/McpToolsTab.vue'
import PromptsTab from '../tabs/PromptsTab.vue'
import SettingsTab from '../tabs/SettingsTab.vue'

interface Props {
  currentTheme: string
  alwaysOnTop: boolean
  audioNotificationEnabled: boolean
  audioUrl: string
  windowWidth: number
  windowHeight: number
  fixedWindowSize: boolean
}

interface Emits {
  themeChange: [theme: string]
  toggleAlwaysOnTop: []
  toggleAudioNotification: []
  updateAudioUrl: [url: string]
  testAudio: []
  stopAudio: []
  testAudioError: [error: any]
  updateWindowSize: [size: { width: number, height: number, fixed: boolean }]
}

defineProps<Props>()
defineEmits<Emits>()

const activeTab = ref('intro')

// 图标加载错误处理
function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement
  // 如果图标加载失败，隐藏图片元素
  img.style.display = 'none'
  console.warn('LOGO图标加载失败，已隐藏')
}
</script>

<template>
  <div class="flex flex-col min-h-screen">
    <!-- 主要内容区域 -->
    <div class="flex-1 flex items-start justify-center p-6 pt-12">
      <div class="max-w-6xl w-full">
        <!-- 标题区域 -->
        <div class="text-center mb-8">
          <!-- 主标题 -->
          <div class="flex items-center justify-center gap-3 mb-3">
            <img
              src="/icons/icon-128.png"
              alt="寸止 Logo"
              class="w-10 h-10 rounded-xl shadow-lg"
              @error="handleImageError"
            >
            <h1 class="text-4xl font-medium text-white">
              寸止
            </h1>
          </div>

          <!-- 服务器状态 -->
          <div class="mb-4">
            <n-tag type="success" size="small" round class="px-3 py-1">
              <template #icon>
                <div class="w-2 h-2 bg-success rounded-full animate-pulse" />
              </template>
              MCP 服务已启动
            </n-tag>
          </div>

          <!-- 副标题 -->
          <p class="text-base opacity-50 font-normal text-white">
            告别AI提前终止烦恼，助力AI更加持久
          </p>
        </div>

        <!-- Tab组件 -->
        <n-tabs v-model:value="activeTab" type="segment" size="small" justify-content="center">
          <n-tab-pane name="intro" tab="介绍">
            <IntroTab />
          </n-tab-pane>
          <n-tab-pane name="mcp-tools" tab="MCP 工具">
            <McpToolsTab />
          </n-tab-pane>
          <n-tab-pane name="prompts" tab="参考提示词">
            <PromptsTab />
          </n-tab-pane>
          <n-tab-pane name="settings" tab="设置">
            <SettingsTab
              :current-theme="currentTheme"
              :always-on-top="alwaysOnTop"
              :audio-notification-enabled="audioNotificationEnabled"
              :audio-url="audioUrl"
              :window-width="windowWidth"
              :window-height="windowHeight"
              :fixed-window-size="fixedWindowSize"
              @theme-change="$emit('themeChange', $event)"
              @toggle-always-on-top="$emit('toggleAlwaysOnTop')"
              @toggle-audio-notification="$emit('toggleAudioNotification')"
              @update-audio-url="$emit('updateAudioUrl', $event)"
              @test-audio="$emit('testAudio')"
              @stop-audio="$emit('stopAudio')"
              @test-audio-error="$emit('testAudioError', $event)"
              @update-window-size="$emit('updateWindowSize', $event)"
            />
          </n-tab-pane>
        </n-tabs>
      </div>
    </div>
  </div>
</template>
