<script setup lang="ts">
import { clipboard } from '@tauri-apps/api'
import ViewerText from './ViewerText.vue'
import ViewerJson from './ViewerJson.vue'

interface StringProps {
  content: string
  readonly?: boolean
}

defineProps<StringProps>()
const selected = ref('text')
const viewRef = ref<InstanceType<typeof ViewerJson|typeof ViewerText>|null>(null)

const views = [
  {
    label: 'Text',
    value: 'text',
    component: ViewerText,
  },
  {
    label: 'Json',
    value: 'json',
    component: ViewerJson,
  },
]

const copyContent = () => {
  const newContent = viewRef.value!.getContent()
  clipboard.writeText(unref(newContent))
}

defineExpose({
  getContent: () => viewRef.value!.getContent(),
  getRowContent: () => viewRef.value!.getRowContent(),
})

const viewComponentMap = computed(() => {
  let viewMap: Record<string, any> = {}
  views.map(view => {
    viewMap[view.value] = view.component
  })

  return viewMap
})

const viewComponent = shallowRef(ViewerText)

const handleChange = (value: string) => {
  selected.value = value
}

watch(selected, value => {
  nextTick(() => {
    viewComponent.value = viewComponentMap.value[value]
  })
})
</script>

<template>
  <div flex flex-col gap-y-4 w-full>
    <el-space>
      <el-select v-model="selected" @change="handleChange">
        <el-option
          v-for="view in views"
          :key="view.value"
          :label="view.label"
          :value="view.value"
        >
          <template #prefix>
            <span>
              <i class="ant-design:clear-outlined" />
            </span>
          </template>
        </el-option>
      </el-select>
      <el-tag>
        Size: {{ content.length }}B
      </el-tag>
      <el-button text size="small" @click="copyContent">
        复制
        <template #icon>
          <span>
            <i class="ant-design:copy-outlined" />
          </span>
        </template>
      </el-button>
    </el-space>

    <component
      :is="viewComponent"
      ref="viewRef"
      :content="content"
      :readonly="readonly"
    />
  </div>
</template>

<style lang="css" scoped>
</style>
