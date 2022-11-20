<script setup lang="ts">


</script>

<template>
  <div relative h-full w-full>
    <div
      class="absolute left-[50%] top-[50%] -translate-[50%]"
      flex
      flex-col
      gap-y-4
      w-full
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
        <el-button size="large" circle text>
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
