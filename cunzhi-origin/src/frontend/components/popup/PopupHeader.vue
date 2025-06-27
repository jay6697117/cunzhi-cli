<script setup lang="ts">
import ThemeIcon from '../common/ThemeIcon.vue'

interface Props {
  currentTheme?: string
  loading?: boolean
  showMainLayout?: boolean
}

interface Emits {
  themeChange: [theme: string]
  openMainLayout: []
}

const props = withDefaults(defineProps<Props>(), {
  currentTheme: 'dark',
  loading: false,
  showMainLayout: false,
})

const emit = defineEmits<Emits>()

function handleThemeChange() {
  // 切换到下一个主题
  const nextTheme = props.currentTheme === 'light' ? 'dark' : 'light'
  emit('themeChange', nextTheme)
}

function handleOpenMainLayout() {
  emit('openMainLayout')
}
</script>

<template>
  <div class="px-4 py-3 select-none">
    <div class="flex items-center justify-between">
      <!-- 左侧：标题 -->
      <div class="flex items-center gap-3">
        <div class="w-3 h-3 rounded-full bg-primary-500" />
        <h1 class="text-base font-medium text-white">
          寸止 - 告别AI提前终止烦恼，助力AI更加持久
        </h1>
      </div>

      <!-- 右侧：操作按钮 -->
      <n-space size="small">
        <n-button
          size="small"
          quaternary
          circle
          :title="props.showMainLayout ? '返回聊天' : '打开设置'"
          @click="handleOpenMainLayout"
        >
          <template #icon>
            <div
              :class="props.showMainLayout ? 'i-carbon-chat' : 'i-carbon-settings'"
              class="w-4 h-4 text-white"
            />
          </template>
        </n-button>
        <n-button
          size="small"
          quaternary
          circle
          :title="`切换到${props.currentTheme === 'light' ? '深色' : '浅色'}主题`"
          @click="handleThemeChange"
        >
          <template #icon>
            <ThemeIcon :theme="props.currentTheme" class="w-4 h-4 text-white" />
          </template>
        </n-button>
      </n-space>
    </div>
  </div>
</template>
