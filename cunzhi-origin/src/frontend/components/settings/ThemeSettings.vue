<script setup>
defineProps({
  currentTheme: {
    type: String,
    required: true,
  },
})

defineEmits(['themeChange'])
</script>

<template>
  <n-card size="small">
    <!-- 卡片头部 -->
    <template #header>
      <n-space align="center">
        <!-- 图标 -->
        <div class="w-10 h-10 rounded-lg bg-primary-100 dark:bg-primary-900 flex items-center justify-center">
          <div class="i-carbon-color-palette text-lg text-primary-600 dark:text-primary-400" />
        </div>

        <!-- 标题和副标题 -->
        <div>
          <div class="text-lg font-medium mb-1 tracking-tight">
            主题设置
          </div>
          <div class="text-sm opacity-60 font-normal">
            选择您喜欢的界面主题
          </div>
        </div>
      </n-space>
    </template>

    <!-- 设置内容 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center">
        <div class="w-1.5 h-1.5 bg-primary-500 rounded-full mr-3 flex-shrink-0" />
        <div>
          <div class="text-sm font-medium leading-relaxed">
            界面主题
          </div>
          <div class="text-xs opacity-60">
            选择浅色或深色主题
          </div>
        </div>
      </div>
      <n-space>
        <!-- 浅色主题 -->
        <n-button
          :type="currentTheme === 'light' ? 'primary' : 'default'"
          size="small"
          @click="$emit('themeChange', 'light')"
        >
          <template #icon>
            <div
              class="w-3 h-3 rounded-full border transition-all duration-200"
              :style="{
                backgroundColor: '#ffffff',
                borderColor: currentTheme === 'light' ? '#14b8a6' : '#d1d5db',
              }"
            />
          </template>
          浅色
        </n-button>

        <!-- 深色主题 -->
        <n-button
          :type="currentTheme === 'dark' ? 'primary' : 'default'"
          size="small"
          @click="$emit('themeChange', 'dark')"
        >
          <template #icon>
            <div
              class="w-3 h-3 rounded-full border transition-all duration-200"
              :style="{
                backgroundColor: '#1f2937',
                borderColor: currentTheme === 'dark' ? '#14b8a6' : '#d1d5db',
              }"
            />
          </template>
          深色
        </n-button>
      </n-space>
    </div>
  </n-card>
</template>
