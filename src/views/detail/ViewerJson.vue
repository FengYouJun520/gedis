<script setup lang="ts">
import { useUiState } from '@/store/ui'
import * as monaco from 'monaco-editor/esm/vs/editor/editor.api'

interface ViewerJsonProps {
  content: string
  readonly?: boolean
  showLineNumber?: boolean
}

const props = defineProps<ViewerJsonProps>()

const message = useMessage()
const uiState = useUiState()
const editorRef = ref<HTMLDivElement>()
let monacoEditor: monaco.editor.IStandaloneCodeEditor

const jsonContent = computed(() => {
  try {
    const obj = JSON.parse(props.content)
    return JSON.stringify(obj, null, 2)
  } catch (error) {
    return props.content
  }
})


watch(() => props.content, () => {
  monacoEditor.setValue(unref(jsonContent))
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
      // vertical line
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

onUnmounted(() => {
  if (monacoEditor) {
    monacoEditor.dispose()
  }
})


defineExpose({
  getContent: () => {
    const content = monacoEditor.getValue()
    return content
  },
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
  <el-card>
    <div ref="editorRef" class="monaco-editor" />
  </el-card>
</template>

<style lang="css" scoped>
.monaco-editor {
	height: calc(100vh - 402px);
  width: 100%;
}
</style>
