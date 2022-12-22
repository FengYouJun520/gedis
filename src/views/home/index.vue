<script setup lang="ts">
import { shell } from '@tauri-apps/api'

const githubUrl = 'https://github.com/FengYouJun520'
const openGithub = () => {
  shell.open(githubUrl)
}
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
        <span text-center>Gedis 0.1.0</span>
      </div>
      <div flex items-center>
        <el-button size="large" circle text @click="openGithub">
          <template #icon>
            <i
              class="mdi:github"
              p3
            />
          </template>
        </el-button>
      </div>
    </div>
  </div>
</template>

<style lang="css" scoped>
</style>
