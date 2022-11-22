import { t } from '@tauri-apps/api/event-2a9960e7'

interface TabsProps {
  id: string
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
    editTab(tabsProps: TabsProps) {
      const index = this.tabs.findIndex(t => t.key === tabsProps.key)
      if (index === -1) {
        return
      }

      this.tabs.splice(index, 1, { ...tabsProps })
    },
    // 删除当前选项卡
    removeTab(key: string) {
      const index = this.tabs.findIndex(t => t.key === key)
      const length = this.tabs.length
      if (index === -1) {
        return
      }

      this.tabs.splice(index, 1)

      /// 如果删除的不是当前激活的选项卡,没必要重新设置
      if (this.currentActive !== key) {
        return
      }

      // 选项卡为空
      if (this.tabs.length === 0) {
        this.currentActive = ''
        return
      }

      // 只有一个选项卡
      if (this.tabs.length === 1) {
        this.currentActive = this.tabs[0].key
        return
      }

      // 至少两个以上选项卡

      // 删除的是第一个选项卡
      if (index === 0) {
        this.currentActive = this.tabs[0].key
      } else if (index === length - 1) {
        // 删除的是最后一个选项卡
        this.currentActive = this.tabs[this.tabs.length - 1].key
      } else {
        // 删除的是中间的选项卡
        this.currentActive = this.tabs[index - 1].key
      }
    },
    // 删除所有属于id的选项卡
    removeTabById(id: string) {
      this.tabs = this.tabs.filter(t => t.id !== id)

      if (this.tabs.length === 1) {
        this.currentActive = this.tabs[0].key
      } else if (this.tabs.length > 1) {
        this.currentActive = this.tabs[this.tabs.length - 1].key
      }
    },
    // 删除其他选项卡
    removeOther(key: string) {
      this.tabs = this.tabs.filter(t => t.key === key)
    },
    // 删除左侧选项卡
    removeLeft(key: string) {
      const index = this.tabs.findIndex(t => t.key === key)
      this.tabs.splice(0, index)
    },
    // 删除右侧侧选项卡
    removeRight(key: string) {
      const index = this.tabs.findIndex(t => t.key === key)
      this.tabs.splice(index + 1)
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
    exist: state => state.tabs.length && state.tabs.length > 0,
  },
})
