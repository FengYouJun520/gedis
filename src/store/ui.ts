
interface UIState {
  theme: string
  asideWidth: number
}

export const useUiState = defineStore('ui-state', {
  state: () : UIState => ({
    asideWidth: 300,
    theme: 'system',
  }),
  actions: {
    changeTheme(newTheme: string) {
      this.theme = newTheme
    },
    setAsideWidth(width: number) {
      this.asideWidth = width
    },
  },
  persist: true,
})
