<script setup lang="ts">
import { useUiState } from '@/store/ui'
import * as monaco from 'monaco-editor/esm/vs/editor/editor.api'

interface ViewerJsonProps {
  readonly?: boolean
  showLineNumber?: boolean
}

const props = defineProps<ViewerJsonProps>()
const content = defineModel<string>({ required: true })
const message = useMessage()
const uiState = useUiState()
const editorRef = ref<HTMLDivElement>()
let monacoEditor: monaco.editor.IStandaloneCodeEditor

const jsonContent = computed(() => {
  try {
    const obj = JSON.parse(unref(content))
    return JSON.stringify(obj, null, 2)
  } catch (error) {
    return unref(content)
  }
})

// 解决初始化显示的是json时内容没有显示的bug
watch(content, () => {
  try {
    monacoEditor.setValue(jsonContent.value)
  } catch (error) {
    return unref(content)
  }
})

watch(() => props.showLineNumber, showLine => {
  nextTick(() => {
    monacoEditor && monacoEditor.updateOptions({
      lineNumbers: showLine ? 'on' : 'off',
    })
  })
})

watchEffect(() => {
  let theme = uiState.theme
  if (theme === 'system') {
    const isDark = usePreferredDark()
    if (unref(isDark)) {
      theme = 'dark'
    } else {
      theme = 'light'
    }
  }
  nextTick(() => {
    monacoEditor && monacoEditor.updateOptions({
      theme: theme === 'dark' ? 'vs-dark' : 'vs',
    })
  })
})

onMounted(() => {
  if (editorRef.value && !monacoEditor) {
    monacoEditor = monaco.editor.create(editorRef.value, {
      value: unref(jsonContent),
      theme: 'vs-dark',
      language: 'json',
      links: false,
      readOnly: props.readonly,
      cursorStyle: props.readonly ? 'underline-thin' : 'line',
      lineNumbers: props.showLineNumber ? 'on' : 'off',
      contextmenu: false,
      tabSize: 2,
      fontSize: 16,
      showFoldingControls: 'always',
      wordWrap: 'on',
      wrappingIndent: 'indent',
      renderLineHighlight: 'none',
      occurrencesHighlight: false,
      scrollBeyondLastLine: false,
      hideCursorInOverviewRuler: true,
      folding: true,
      colorDecorators: false,
      minimap: {
        enabled: true,
      },
      guides: {
        indentation: true,
        highlightActiveIndentation: true,
        bracketPairs: true,
      },
      scrollbar: {
        useShadows: false,
        verticalScrollbarSize: 10,
        horizontalScrollbarSize: 10,
      },
      wordWrapOverride2: 'off',
    })
  }
})

onUnmounted(() => monacoEditor && monacoEditor.dispose())


defineExpose({
  getContent: () => monacoEditor.getValue(),
  getRowContent: () => {
    const content = monacoEditor.getValue()
    try {
      return JSON.stringify(JSON.parse(unref(content)), null, 0)
    } catch (error: any) {
      message.error(error.message)
      return ''
    }
  },
})

useResizeObserver(document.body, () => {
  monacoEditor && monacoEditor.layout()
})


watch(() => uiState.theme, newTheme => {
  let theme = newTheme
  if (theme === 'system') {
    const isDark = usePreferredDark()
    if (unref(isDark)) {
      theme = 'dark'
    } else {
      theme = 'light'
    }
  }

  nextTick(() => {
    monacoEditor && monacoEditor.updateOptions({
      theme: theme === 'dark' ? 'vs-dark' : 'vs',
    })
  })

  return theme
})
</script>

<template>
  <n-card>
    <div ref="editorRef" class="monaco-editor" />
  </n-card>
</template>

<style lang="css" scoped>
.monaco-editor {
	height: calc(100vh - 402px);
  width: 100%;
}
</style>
