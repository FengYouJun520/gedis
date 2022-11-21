import { MousePressedOptions } from '@vueuse/core'

// 动态改变侧边栏的宽度
export function useResizeAsideWidth(options: MousePressedOptions | undefined, handleMouse: (event: MouseEvent) => void) {
  const { pressed } = useMousePressed(options)

  watch(pressed, () => {
    if (unref(pressed)) {
      window.addEventListener('mousemove', handleMouse)
    } else {
      window.removeEventListener('mousemove', handleMouse)
    }
  })

  return { pressed }
}
