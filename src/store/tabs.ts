import { tabBarProps } from 'element-plus'

interface TabsProps {
  key: string
  name: string
  db: number
  path: string
  icon?: string
}

interface TabsState {
  tabs: Map<string, TabsProps>
}

export const useTabs = defineStore('tabs', {
  state: () : TabsState => ({
    tabs: new Map(),
  }),
  actions: {
    addTab(key: string, tabsProps: TabsProps) {
      if (this.tabs.has(key)) {
        return
      }

      this.tabs.set(key, tabsProps)
    },
    removeTab(key: string) {
      this.tabs.delete(key)
    },

  },
  getters: {
    getTabs: state => Array.from(state.tabs.values()),
  },
})
