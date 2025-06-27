<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { onMounted, ref } from 'vue'

interface ReplyConfig {
  enable_continue_reply: boolean
  auto_continue_threshold: number
  continue_prompt: string
}

const localConfig = ref<ReplyConfig>({
  enable_continue_reply: true,
  auto_continue_threshold: 1000,
  continue_prompt: '请按照最佳实践继续',
})

// 加载配置
async function loadConfig() {
  try {
    const config = await invoke('get_reply_config')
    localConfig.value = config as ReplyConfig
  }
  catch (error) {
    console.error('加载继续回复配置失败:', error)
  }
}

// 更新配置
async function updateConfig() {
  try {
    await invoke('set_reply_config', { replyConfig: localConfig.value })
  }
  catch (error) {
    console.error('保存继续回复配置失败:', error)
  }
}

onMounted(() => {
  loadConfig()
})
</script>

<template>
  <n-card size="small">
    <!-- 卡片头部 -->
    <template #header>
      <n-space align="center">
        <!-- 图标 -->
        <div class="w-10 h-10 rounded-lg bg-info/10 dark:bg-info/20 flex items-center justify-center">
          <div class="i-carbon-repeat text-lg text-blue-600 dark:text-blue-400" />
        </div>

        <!-- 标题和副标题 -->
        <div>
          <div class="text-lg font-medium mb-1 tracking-tight">
            继续回复设置
          </div>
          <div class="text-sm opacity-60 font-normal">
            配置 AI 回复继续行为
          </div>
        </div>
      </n-space>
    </template>

    <!-- 设置内容 -->
    <n-space vertical size="large">
      <!-- 启用继续回复 -->
      <div class="flex items-center justify-between">
        <div class="flex items-center">
          <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 flex-shrink-0" />
          <div>
            <div class="text-sm font-medium leading-relaxed">
              启用继续回复
            </div>
            <div class="text-xs opacity-60">
              启用后将显示继续按钮
            </div>
          </div>
        </div>
        <n-switch
          v-model:value="localConfig.enable_continue_reply"
          size="small"
          @update:value="updateConfig"
        />
      </div>

      <!-- 继续提示词 -->
      <div v-if="localConfig.enable_continue_reply">
        <div class="flex items-center mb-3">
          <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 flex-shrink-0" />
          <div>
            <div class="text-sm font-medium leading-relaxed">
              继续提示词
            </div>
            <div class="text-xs opacity-60">
              点击继续按钮时发送的提示词
            </div>
          </div>
        </div>
        <n-input
          v-model:value="localConfig.continue_prompt"
          size="small"
          placeholder="请按照最佳实践继续"
          @input="updateConfig"
        />
      </div>
    </n-space>
  </n-card>
</template>
