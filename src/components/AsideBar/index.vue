<script setup lang="ts">
const selectDB = ref(0)

const handleClick = (event: MouseEvent) => {
  event.stopPropagation()
}

const handleOpen = (index: string, indexPath: string) => {
  console.log(`open index: ${index}, indexPath: ${indexPath}`)
}

const handleclose = (index: string, indexPath: string) => {
  console.log(`close index: ${index}, indexPath: ${indexPath}`)
}

const handleChange = (value: any) => {
  console.log(value)
}

const options = [
  {
    value: 0,
    label: 'DB0',
  },
  {
    value: 1,
    label: 'DB1',
  },
]
</script>

<template>
  <el-aside h-full>
    <el-scrollbar view-class="h-full">
      <div flex flex-col h-full>
        <div flex justify-center items-center gap-x4 p4>
          <el-button type="primary" text bg class="flex-1">
            <template #icon>
              <i class="ic:round-add-circle" />
            </template>
            <span class="text-base">新建连接</span>
          </el-button>
          <el-space>
            <el-button size="small" text bg>
              <template #icon>
                <i class="material-symbols:settings" />
              </template>
            </el-button>
            <el-button size="small" text bg>
              <template #icon>
                <i class="mdi:clock-minus-outline" />
              </template>
            </el-button>
          </el-space>
        </div>
        <div flex-1>
          <el-menu
            h-full
            @select="handleChange"
            @open="handleOpen"
            @close="handleclose"
          >
            <el-sub-menu index="1">
              <template #title>
                <div flex-1 flex justify-between items-center mr6>
                  <span>localhost</span>
                  <div flex items-center gap-x1>
                    <i
                      class="mdi:home w20px h20px hover:(text-[var(--el-menu-hover-text-color)])"
                      @click="handleClick"
                    />
                    <i
                      class="mdi:console w20px h20px hover:(text-[var(--el-menu-hover-text-color)])"
                      @click="handleClick"
                    />
                    <i
                      class="material-symbols:refresh w20px h20px hover:(text-[var(--el-menu-hover-text-color)])"
                      @click="handleClick"
                    />
                    <el-dropdown>
                      <i
                        class="icon-park-twotone:more-app w16px h16px hover:(text-[var(--el-menu-hover-text-color)])"
                        @click="handleClick"
                      />
                      <template #dropdown>
                        <el-dropdown-menu>
                          <el-dropdown-item>Action 1</el-dropdown-item>
                          <el-dropdown-item>Action 2</el-dropdown-item>
                          <el-dropdown-item>Action 3</el-dropdown-item>
                          <el-dropdown-item disabled>
                            Action 4
                          </el-dropdown-item>
                          <el-dropdown-item divided>
                            Action 5
                          </el-dropdown-item>
                        </el-dropdown-menu>
                      </template>
                    </el-dropdown>
                  </div>
                </div>
              </template>
              <div flex flex-col gap-y2 px2 my2>
                <div flex items-center gap-x2>
                  <el-select v-model="selectDB" placeholder="Select" size="large">
                    <el-option
                      v-for="item in options"
                      :key="item.value"
                      :label="item.label"
                      :value="item.value"
                    />
                  </el-select>
                  <el-button text bg>
                    新增key
                  </el-button>
                </div>
                <div>
                  <el-input placeholder="搜索" />
                </div>
              </div>
              <el-menu-item index="list key">
                <span>list key</span>
              </el-menu-item>
            </el-sub-menu>
          </el-menu>
        </div>
      </div>
    </el-scrollbar>
  </el-aside>
</template>

<style lang="css" scoped>
</style>
