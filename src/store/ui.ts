
const MIN_WIDTH = 280
const MAX_WIDTH = 800

interface UIState {
  theme: string
  asideWidth: number
}

export const useUiState = defineStore('ui-state', {
  state: () : UIState => ({
    asideWidth: MIN_WIDTH,
    theme: 'system',
  }),
  actions: {
    changeTheme(newTheme: string) {
      this.theme = newTheme
    },
    setAsideWidth(width: number) {
      if (width > MIN_WIDTH && width < MAX_WIDTH) {
        this.asideWidth = width
      }
    },
  },
  persist: true,
})
