<script setup lang="ts">
import { useUiState } from '@/store/ui'
import * as monaco from 'monaco-editor/esm/vs/editor/editor.api'

interface ViewerJsonProps {
  content: string
  readonly?: boolean
}

const props = defineProps<ViewerJsonProps>()
const uiState = useUiState()
const editorRef = ref<HTMLDivElement | null>(null)
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


onMounted(() => {
  if (editorRef.value && !monacoEditor) {
    monacoEditor = monaco.editor.create(editorRef.value!, {
      value: unref(jsonContent),
      theme: 'vs-dark',
      language: 'json',
      links: false,
      readOnly: props.readonly,
      cursorStyle: props.readonly ? 'underline-thin' : 'line',
      lineNumbers: 'off',
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
      ElMessage.error(error.message)
      return ''
    }
  },
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
  <div class="text-formated-container">
    <el-card>
      <div ref="editorRef" class="monaco-editor" />
    </el-card>
  </div>
</template>

<style lang="css" scoped>
.monaco-editor {
  min-height: 350px;
  min-width: 350px;
	height: 100%;
  width: 100%;
}


.monaco-editor :deep(.decorationsOverviewRuler) {
  display: none !important;
}
</style>
