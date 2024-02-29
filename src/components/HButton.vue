<script setup lang="ts">
defineOptions({
  name: 'HButton',
})

type Props = {
  disabled?: boolean
}

defineProps<Props>()
</script>

<template>
  <button type="button"
          :disabled="disabled"
          class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800">
    <slot />
  </button>
</template>
