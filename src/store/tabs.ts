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
}

export const useTabs = defineStore('tabs', {
  state: () : TabsState => ({
    tabs: [],
  }),
  actions: {
    addTab(tabsProps: TabsProps) {
      if (this.tabs.findIndex(t => t.key === tabsProps.key) !== -1) {
        return
      }

      this.tabs.push({ ...tabsProps })
    },
    removeTab(key: string) {
      const index = this.tabs.findIndex(t => t.key === key)
      if (index === -1) {
        return
      }

      this.tabs.splice(index, 1)
    },

  },
  getters: {
    getTabs: state => state.tabs,
  },
})
