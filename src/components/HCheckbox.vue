<script setup lang="ts" generic="T extends boolean">
defineOptions({
  name: 'HInput',
})

const model = defineModel<boolean>({ default: false })

defineProps<{ label: string }>()
</script>

<template>
  <div class="block">
    <div class="mt-2">
      <div>
        <label class="inline-flex items-center cursor-pointer">
          <input type="checkbox" class="rounded border-gray-300 text-indigo-600 shadow-sm focus:border-indigo-300 focus:ring focus:ring-offset-0 focus:ring-indigo-200 focus:ring-opacity-50" v-model="model">
          <span class="ml-2">{{ label }}</span>
        </label>
      </div>
    </div>
  </div>
</template>
