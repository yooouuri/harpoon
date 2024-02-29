<script setup lang="ts" generic="T extends string">
defineOptions({
  name: 'HInput',
})

const model = defineModel<string>({ default: '' })

defineProps<{ label: string }>()
</script>

<template>
  <label class="block">
    <span class="text-gray-700">{{ label }}</span>
    <input type="text"
           class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
           placeholder=""
           v-model="model">
  </label>
</template>
