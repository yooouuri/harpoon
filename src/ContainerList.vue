<script setup lang="ts">
import { onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import ContainerListItem from '@/components/ContainerListItem.vue'
import { useContainersStore } from '@/stores/containers'

type Container = {
  Id: string
  Image: string
}

const router = useRouter()

async function openContainer(id: string) {
  await router.push(`/containers/${id}`)
}

// const containerUpdateListener = await listen<Container>('container-update', (event) => {
//   // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
//   // event.payload is the payload object
// })

// onUnmounted(() => {
//   containerUpdateListener()
// })

const containerStore = useContainersStore()

containerStore.containers = (await invoke('list_containers')).map((container) => ({
  id: container.Id,
  name: container.Names.map(name => name.slice(1)).join(', '),
  image: container.Image
}))
</script>

<template>
  Create a new container

  <div class="grid gap-4">
    <ContainerListItem v-for="item in containerStore.containers"
                       :id="item.id"
                       :name="item.name"
                       :image="item.image"
                       :key="item.id"
                       @open="openContainer" />
  </div>
</template>
