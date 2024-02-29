<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { useContainersStore } from '@/stores/containers'
import { invoke } from '@tauri-apps/api/tauri'
import { listen, emit, type UnlistenFn } from '@tauri-apps/api/event'
// import { appWindow } from '@tauri-apps/api/window'
import { Terminal } from 'xterm'
import { FitAddon } from '@xterm/addon-fit'

const route = useRoute()

const containerStore = useContainersStore()

const container = containerStore.containers.find(container => container.id === route.params.id)

const command = ref('')

let execListener: UnlistenFn

onMounted(async () => {
  const term = new Terminal()
  const fitAddon = new FitAddon() // TODO resize terminal on window resize
  term.loadAddon(fitAddon)
  term.open(document.getElementById('terminal') as HTMLElement)
  term.onKey((e) => {
    if (e.key == '\r') {
      term.write('\r\n');

      invoke('run_command', { containerName: container?.id, command: `${command.value}\n` }).then(() => console.log('command send to rust'))

      command.value = ''
    } else if (e.domEvent.key === 'Backspace') {
      command.value = command.value.slice(0, command.value.length - 1);

      term.write('\b \b');
    } else {
      term.write(e.key);

      command.value += e.key
    }
  })

  fitAddon.fit()

  execListener = await listen<string>('exec-output', (event) => {
    term.write(event.payload)
  })

  await invoke('create_exec_listener', { containerName: container?.id })

  // TODO fix resize, its janky for now...
  // await appWindow.onResized(() => {
  //   fitAddon.fit()
  // })
})

onUnmounted(async () => {
  execListener()
})
</script>

<template>
  <div class="w-full h-full">
    <div id="terminal" class="bg-red-500 w-full h-full"></div>
  </div>
</template>

<style lang="scss">
@import url('xterm/css/xterm.css');
</style>
