<script setup lang="ts">
interface Feature {
  icon: string
  title: string
  subtitle: string
  iconBg: string
  features: string[]
}

defineProps<{
  feature: Feature
}>()
</script>

<template>
  <n-card size="small">
    <!-- 卡片头部 -->
    <template #header>
      <n-space align="center">
        <!-- 图标 -->
        <div
          class="w-10 h-10 rounded-lg flex items-center justify-center" :class="[
            feature.iconBg,
            feature.iconBg.includes('blue') ? 'dark:bg-blue-900'
            : feature.iconBg.includes('purple') ? 'dark:bg-purple-900'
              : feature.iconBg.includes('green') ? 'dark:bg-green-900'
                : 'dark:bg-gray-900',
          ]"
        >
          <!-- UnoCSS图标 -->
          <div
            :class="feature.icon"
          />
        </div>

        <!-- 标题和副标题 -->
        <div>
          <div class="text-lg font-medium mb-1 tracking-tight">
            {{ feature.title }}
          </div>
          <div class="text-sm opacity-60 font-normal">
            {{ feature.subtitle }}
          </div>
        </div>
      </n-space>
    </template>

    <!-- 功能列表 -->
    <n-space vertical size="small">
      <div v-for="(item, index) in feature.features" :key="index" class="flex items-center text-sm leading-relaxed">
        <div class="w-1.5 h-1.5 bg-green-500 rounded-full mr-3 flex-shrink-0" />
        <span class="opacity-90">{{ item }}</span>
      </div>
    </n-space>
  </n-card>
</template>
