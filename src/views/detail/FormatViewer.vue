<script setup lang="ts">
import { clipboard } from '@tauri-apps/api'
import ViewerText from './ViewerText.vue'
import ViewerJson from './ViewerJson.vue'
import { SelectOption } from 'naive-ui'

interface FormatViewProps {
  readonly?: boolean
  showFormat?: string
}

const props = defineProps<FormatViewProps>()
const content = defineModel<string>({ required: true })
const selectComponent = ref(props.showFormat || 'text')
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

const viewComponent = shallowRef(selectComponent.value === 'text' ? ViewerText : ViewerJson)

watch(selectComponent, value => {
  nextTick(() => {
    viewComponent.value = viewComponentMap.value[value]
  })
})

const options = computed<SelectOption[]>(() => views.map(view => ({
  label: view.label,
  value: view.value,
})))
</script>

<template>
  <div w-full flex flex-col space-y-5>
    <n-grid :x-gap="12" :cols="24" responsive="screen" item-responsive>
      <n-gi span="8">
        <n-select v-model:value="selectComponent" :options="options" />
      </n-gi>
      <n-gi span="16" flex items-center space-x-2>
        <n-tag type="primary" :bordered="false">
          Size: {{ content.length }}B
        </n-tag>
        <n-button tertiary size="small" @click="copyContent">
          复制
          <template #icon>
            <span>
              <i class="ant-design:copy-outlined" />
            </span>
          </template>
        </n-button>
        <template v-if="selectComponent === 'json'">
          <span>显示行号: </span>
          <n-switch v-model:value="lineNumber" />
        </template>
      </n-gi>
    </n-grid>

    <component
      :is="viewComponent"
      ref="viewRef"
      v-model="content"
      :readonly="readonly"
      :show-line-number="lineNumber"
    />
  </div>
</template>

<style lang="css" scoped>
</style>
