<script setup lang="ts">
import type { McpRequest } from '../../types/popup'
import { useMagicKeys } from '@vueuse/core'
import { computed, watch } from 'vue'
import { useKeyboard } from '../../composables/useKeyboard'

interface Props {
  request: McpRequest | null
  loading?: boolean
  submitting?: boolean
  canSubmit?: boolean
  connectionStatus?: string
  continueReplyEnabled?: boolean
  inputStatusText?: string
}

interface Emits {
  submit: []
  continue: []
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  submitting: false,
  canSubmit: false,
  connectionStatus: '已连接',
  continueReplyEnabled: true,
  inputStatusText: '',
})

const emit = defineEmits<Emits>()

// 跨平台快捷键支持
const keys = useMagicKeys()
const ctrlEnter = keys['Ctrl+Enter']
const metaEnter = keys['Meta+Enter']

// 使用统一的键盘处理
const { isMac } = useKeyboard()

const shortcutText = computed(() => {
  return isMac.value ? '⌘+回车 快速发送' : 'Ctrl+回车 快速发送'
})

const statusText = computed(() => {
  // 如果可以提交，直接显示快捷键提示
  if (props.canSubmit) {
    return shortcutText.value
  }

  // 如果有输入状态文本且不是默认状态，显示输入状态
  if (props.inputStatusText && props.inputStatusText !== '等待输入...') {
    return props.inputStatusText
  }

  // 根据请求类型显示不同的提示
  if (props.request?.predefined_options) {
    return '选择选项或输入文本'
  }
  return '请输入内容'
})

// 处理快捷键
watch([ctrlEnter, metaEnter], ([ctrlPressed, metaPressed]) => {
  if ((ctrlPressed || metaPressed) && props.canSubmit && !props.submitting) {
    handleSubmit()
  }
})

function handleSubmit() {
  if (props.canSubmit && !props.submitting) {
    emit('submit')
  }
}

function handleContinue() {
  if (!props.submitting) {
    emit('continue')
  }
}
</script>

<template>
  <div class="px-4 py-3 bg-gray-100 min-h-[60px] select-none">
    <div v-if="!loading" class="flex justify-between items-center">
      <!-- 左侧状态信息 -->
      <div class="flex items-center">
        <div class="flex items-center gap-2 text-xs text-gray-600">
          <div class="w-2 h-2 rounded-full bg-primary-500" />
          <span class="font-medium">{{ connectionStatus }}</span>
          <span class="opacity-60">|</span>
          <span class="opacity-60">{{ statusText }}</span>
        </div>
      </div>

      <!-- 右侧操作按钮 -->
      <div class="flex items-center">
        <n-space size="small">
          <!-- 继续按钮 -->
          <n-button
            v-if="continueReplyEnabled"
            :disabled="submitting"
            :loading="submitting"
            size="medium"
            type="default"
            @click="handleContinue"
          >
            <template #icon>
              <div class="i-carbon-play w-4 h-4" />
            </template>
            继续
          </n-button>

          <!-- 发送按钮 -->
          <n-button
            type="primary"
            :disabled="!canSubmit || submitting"
            :loading="submitting"
            size="medium"
            @click="handleSubmit"
          >
            <template #icon>
              <div v-if="!submitting" class="i-carbon-send w-4 h-4" />
            </template>
            {{ submitting ? '发送中...' : '发送' }}
          </n-button>
        </n-space>
      </div>
    </div>
  </div>
</template>
