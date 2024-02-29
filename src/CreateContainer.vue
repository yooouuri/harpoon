<script setup lang="ts">
import { reactive } from 'vue'

type ContainerPortMapping = {
  containerPort: number
  hostPort: number
  protocol: string
}

type VolumeMapping = {
  containerPath: string
  hostPath: string
  readOnly: boolean
}

type Container = {
  name: string
  platform: string
  image: string
  ports: ContainerPortMapping[]
  labels: string[]
  volumes: VolumeMapping[]
  cmd: string | null
  envs: string[]
}

const container = reactive<Container>({
  name: '', // /?[a-zA-Z0-9][a-zA-Z0-9_.-]+
  platform: '', // os[/arch[/variant]]
  image: '',
  ports: [],
  labels: [],
  volumes: [],
  cmd: null,
  envs: [],
})
</script>

<template>
  <div>
    <input type="checkbox" class="form-checkbox rounded text-pink-500" />
  </div>
</template>
