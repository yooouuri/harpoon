<script setup lang="ts">
import { useRoute } from 'vue-router'
import { useContainersStore } from '@/stores/containers'
import { onMounted, onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen, emit, type UnlistenFn } from '@tauri-apps/api/event'

const route = useRoute()

const containerStore = useContainersStore()

const container = containerStore.containers.find(container => container.id === route.params.id)

const logs = ref<string | null>(null)

let containerLogsListener: UnlistenFn | null = null

const logging = ref(false)

async function pauseLogging() {
  logging.value = false

  if (containerLogsListener !== null) {
    containerLogsListener()
  }

  await emit('stop-logs')
}

async function resumeLogging() {
  await invoke('container_logs', { containerName: container?.id })

  containerLogsListener = await listen<string>('container-logs', (event) => {
    logs.value += event.payload
  })

  logging.value = true
}

onUnmounted(async () => {
  if (containerLogsListener !== null) {
    containerLogsListener()
  }

  await emit('stop-logs')
})
</script>

<template>
  <div>
    <div class="h-96 overflow-auto border p-2 my-2 mr-2 rounded group">
      <div class="hidden group-hover:flex fixed right-6 gap-1">
        <div class="icon-pause cursor-pointer" @click="pauseLogging" v-if="logging"></div>
        <div class="icon-play cursor-pointer" @click="resumeLogging" v-else></div>
        <div class="icon-trash-2 cursor-pointer" @click="logs = ''"></div>
      </div>
      <div class="h-fit w-fit icon-loader-2 text-3xl animate-spin" v-if="logs === null && logging === true"></div>
      <pre v-else>{{ logs }}</pre>
    </div>
  </div>
</template>
