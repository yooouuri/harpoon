<script setup lang="ts">
import { computed, defineAsyncComponent, ref } from 'vue'
import { useRoute } from 'vue-router'
import { useContainersStore } from '@/stores/containers'

const route = useRoute()

const containerStore = useContainersStore()

const container = containerStore.containers.find(container => container.id === route.params.id)

const tabs = [
  {
    name: 'logs',
    component: defineAsyncComponent(() => import('@/components/ContainerLogs.vue')),
  },
  {
    name: 'exec',
    component: defineAsyncComponent(() => import('@/components/ContainerExecs.vue')),
  },
  {
    name: 'stats',
    component: defineAsyncComponent(() => import('@/components/ContainerStats.vue')),
  },
  {
    name: 'files',
    component: defineAsyncComponent(() => import('@/components/ContainerFiles.vue')),
  },
]

const activeTab = ref('logs')

const activeTabComponent = computed(() => tabs.find(tab => tab.name === activeTab.value)?.component)
</script>

<template>
  <div class="z-0 absolute top-0 h-16 flex gap-1 flex-col justify-center w-full border-b border-b-slate-200">
    <div class="m-10">
      <div>{{ container.name }}</div>
      <div>{{ container.image }}</div>
    </div>
  </div>
  <nav class="w-full mt-1">
    <ul class="flex">
      <li class="p-4 pb-3 mb-1 cursor-pointer" :class="{ 'border-b-4 border-b-slate-200': activeTab === 'logs' }" @click="activeTab = 'logs'">Logs</li>
      <li class="p-4 pb-3 mb-1 cursor-pointer" :class="{ 'border-b-4 border-b-slate-200': activeTab === 'exec' }" @click="activeTab = 'exec'">Interactive shell</li>
      <li class="p-4 pb-3 mb-1 cursor-pointer" :class="{ 'border-b-4 border-b-slate-200': activeTab === 'stats' }" @click="activeTab = 'stats'">Stats</li>
      <li class="p-4 pb-3 mb-1 cursor-pointer" :class="{ 'border-b-4 border-b-slate-200': activeTab === 'files' }" @click="activeTab = 'files'">Files</li>
    </ul>
  </nav>
  <component :is="activeTabComponent" />
</template>
