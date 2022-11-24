export interface TerminalProps {
  // Terminal实例名称，同一页面的name必须唯一，Api中使用也需用到此值, default: terminal
  name?: string
  // 初始化上下文文本, default: /vue-web-terminal
  context?: string
  // header中显示的标题, default: vue-web-terminal
  title?: string
  // 是否显示header，此开关会影响拖拽功能, default: true
  showHeader?:boolean
  // Terminal初始化时显示的日志，是由消息对象组成的数组，设为null则不显示
  initLog?: Message[] | null
  // 当前Terminal显示的日志条数超出此限制会发出警告，设一个<= 0的值将不发出警告, default: 200
  warnLogCountLimit?: number
  // 是否打开命令行自动搜索提示功能, default: true
  autoHelp?: boolean
  // 是否显示右上角命令样例提示，前提是开启了auto-help, default: true
  enableExampleHint?:boolean
  // 自定义的命令库，搜索提示功能会扫描此库，见命令定义格式, 默认内置命令
  commandStore?: Command[]
  // 命令行库排序，自定义命令库的显示排序规则
  commandStoreSort?: (a: Command, b: Command) => number
  // 自定义输入过滤，返回值为过滤后的字符串，必须是纯文本，不能带html标签,
  // default: function(当前输入字符char, 输入框内字符串value, input事件event)
  inputFilter?: (c: string, value: string, event: InputEvent) => string
  // 拖拽窗口配置项，如果不配置此项宽高将会100%填充父元素，窗口宽高等同于父元素宽高
  dragConf?: DragConf
  // 命令显示格式化函数，一般用于输入命令高亮显示，传入当前命令返回新的命令，支持html。
  // 如果不设置将使用内部定义的高亮样式
  commandFormater?: (cmd: Command) => Command
}

export interface Message {
  // 必填，消息内容，不同消息格式的内容格式不一样，具体规则见下文
  // string、json、object、array
  content: string | Record<string, any> | any[]
  // 消息格式类型，默认值为normal
  type?: string
  // 消息级别，仅类型为normal有效, success、error、system、info、warning
  class?: string
  // 标签，仅类型为normal有效
  tag?: string
}

export interface Command {
  // 命令关键字，必填
  key: string
  // 显示标题
  title?: string
  // 分组，可自定义，内置的help命令可以按照此字段进行筛选
  group?: string
  // 使用方法
  usage?: string
  // 详细描述
  description?: string
  // 使用示例，见命令示例格式
  // [
  //   {
  //     "des": "获取所有任务信息",
  //     "cmd": "task -o pack"
  //   },
  //   {
  //     "des": "获取任务进度",
  //     "cmd": "task -o query"
  //   }
  // ]
  example: any[]
}

export interface DragConf {
  // 拖拽窗口宽度，可以是数字（单位px）也可以是百分比（相对于浏览器窗口）
  width: number|string
  // 拖拽窗口高度，同宽度
  height: number|string
  // 窗口层级，默认100
  zindex?: number
  // 窗口初始化位置，如果不填则默认位置在浏览器窗口中央，其中x和y的单位为px， {"x": 700, "y": 500}
  init: Record<string, number|string>
}

export type EventMessage = (message: string | Message) => void
