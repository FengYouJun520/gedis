<script setup lang="ts">
import { shell, app } from '@tauri-apps/api'

const githubUrl = 'https://github.com/FengYouJun520/gedis'
const openGithub = () => {
  shell.open(githubUrl)
}

const projectName = ref('')
const version = ref('')
onMounted(async () => {
  projectName.value = await app.getName()
  version.value = await app.getVersion()
})
</script>

<template>
  <div h-full w-full>
    <div
      class="absolute left-[50%] top-[50%] -translate-[50%]"
      flex
      flex-col
      gap-y-4
      w-full
      h-full
      justify-center
      items-center
    >
      <i
        class="logos:redis"
        w200px
        h200px
      />
      <div>
        <n-tag type="success" size="large">
          {{ projectName }}: {{ version }}
        </n-tag>
      </div>
      <div flex items-center>
        <n-button size="large" circle tertiary @click="openGithub">
          <template #icon>
            <i
              class="mdi:github"
              p3
            />
          </template>
        </n-button>
      </div>
    </div>
  </div>
</template>

<style lang="css" scoped>
</style>
