interface TabsProps {
  key: string
  name: string
  db: number
  path: string
  icon?: string
  query: Record<string, any>
}

interface TabsState {
  tabs: TabsProps[]
  currentActive: string
}

export const useTabs = defineStore('tabs', {
  state: () : TabsState => ({
    tabs: [],
    currentActive: '',
  }),
  actions: {
    addTab(tabsProps: TabsProps) {
      this.setActive(tabsProps.key)

      if (this.tabs.findIndex(t => t.key === tabsProps.key) !== -1) {
        return
      }

      this.tabs.push({ ...tabsProps })
    },
    removeTab(key: string) {
      const index = this.tabs.findIndex(t => t.key === key)
      const length = this.tabs.length
      if (index === -1) {
        return
      }

      this.tabs.splice(index, 1)

      if (this.tabs.length === 0) {
        this.currentActive = ''
        return
      }

      if (this.tabs.length === 1) {
        this.currentActive = this.tabs[0].key
        return
      }

      if (index === 0) {
        this.currentActive = this.tabs[0].key
      } else if (index === length - 1) {
        this.currentActive = this.tabs[this.tabs.length - 1].key
      } else {
        this.currentActive = this.tabs[index - 1].key
      }
    },
    getTab(key: string) {
      return this.tabs.find(t => t.key === key)
    },
    setActive(active: string) {
      this.currentActive = active
    },
  },
  getters: {
    getTabs: state => state.tabs,
    exist: state => !!state.tabs.length,
  },
})
