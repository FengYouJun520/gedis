<script setup lang="ts">
import { clipboard } from '@tauri-apps/api'
import ViewerText from './ViewerText.vue'
import ViewerJson from './ViewerJson.vue'

interface FormatViewProps {
  content: string
  readonly?: boolean
  selected?: string
}

const props = defineProps<FormatViewProps>()

const selectComponent = ref('text')
const viewRef = ref<InstanceType<typeof ViewerJson|typeof ViewerText>|null>(null)
const lineNumber = ref(false)

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

watch(props, newprops => {
  selectComponent.value = newprops.selected || 'text'
})

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
  selectComponent.value = value
}

watch(selectComponent, value => {
  nextTick(() => {
    viewComponent.value = viewComponentMap.value[value]
  })
})
</script>

<template>
  <div w-full flex flex-col space-y-5>
    <div flex items-center space-x-2>
      <el-select v-model="selectComponent" @change="handleChange">
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
      <span>显示行号: </span>
      <el-switch v-model="lineNumber" />
    </div>

    <component
      :is="viewComponent"
      ref="viewRef"
      :content="content"
      :readonly="readonly"
      :show-line-number="lineNumber"
    />
  </div>
</template>

<style lang="css" scoped>
</style>
