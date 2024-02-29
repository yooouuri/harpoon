import { reactive, toRefs } from 'vue'
import { defineStore } from 'pinia'

export const useContainersStore = defineStore('containers', () => {
  const state = reactive({
    containers: [],
  })

  return {
    ...toRefs(state),
  }
})
