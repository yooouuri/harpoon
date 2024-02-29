<script setup lang="ts">
type Props = {
  id: string
  name: string
  image: string
}

defineProps<Props>()

const emit = defineEmits<{
  (e: 'open', id: string): void
}>()
</script>

<template>
  <div class="flex p-4 bg-red-400 gap-x-2 w-full" @click="emit('open', id)">
    <div>{{ name }}</div>
    <div>{{ image }}</div>
    <div>Running</div>
    <div>80:80</div>
    <div>
      <div class="icon-terminal-square"></div>
    </div>
  </div>
</template>
