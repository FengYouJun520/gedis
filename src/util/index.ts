import { Keyspace } from '@/types/redis'

/**
 * https://github.com/qishibo/AnotherRedisDesktopManager/blob/c2643301d1787bfcdca25014d5a21c423b1ea35b/src/util.js#L202
 * @param keys all key
 * @param separator default ':'
 * @returns TreeNode
 */
export function keysToTree(keys: string[], separator = ':') {
  const tree: any = {}

  keys.forEach(key => {
    let currentNode = tree

    const keys = key.split(separator)
    const lastIndex = keys.length - 1

    keys.forEach((value, index) => {
      // key node
      if (index === lastIndex) {
        currentNode[`${key}\`k\``] = {
          isLeaf: true,
          label: key,
        }
      } else {
        // folder node
        currentNode[value] === undefined && (currentNode[value] = {})
      }

      currentNode = currentNode[value]
    })
  })


  // to tree format
  return formatTreeData(tree, '', separator)
}

function formatTreeData(tree: any, previousKey = '', separator = ':') {
  return Object.keys(tree).map(key => {
    const node: any = { label: key ? key : '[Empty]', key }

    // folder node
    if (!tree[key].isLeaf && Object.keys(tree[key]).length > 0) {
      // value
      const tillNowKeylabel = previousKey + key + separator

      /*
       * folder's value may same with key label, such as 'aa-'
       * for unique, add 'F' prefix
       */
      node.key = `F${tillNowKeylabel}`
      node.children = formatTreeData(tree[key], tillNowKeylabel, separator)
      node.keyCount = node.children.reduce((a: any, b: any) => a + (b.keyCount || 1), 0)
      /*
       * too many children, force cut, do not incluence keyCount display
       * node.open && node.children.length > forceCut && node.children.splice(forceCut);
       * keep folder node in front of the tree and sorted(not include the outest list)
       * async sort, only for opened folders
       */
      sortKeysAndFolder(node.children)
      node.value = tillNowKeylabel
    } else {
      // key node
      node.label = key.replace(/`k`$/, '')
      node.key = node.label.toString()
      node.value = key.replace(/`k`$/, '')
    }

    return node
  })
}
/*
 * nodes is reference, keep folder in front and sorted,
 * keep keys in tail and sorted
 * sortByData
 */
function sortKeysAndFolder(nodes: any[]) {
  nodes.sort((a, b) => {
    // a & b are all keys
    if (!a.children && !b.children) {
      return a.label > b.label ? 1 : -1
    } else if (a.children && b.children) {
      // a & b are all folder
      return a.label > b.label ? 1 : -1
    } else if (a.children) {
      // a is folder, b is key
      return -1
    } else {
      // a is key, b is folder
      return 1
    }
  })
}


const keyspaceMatch = /keys=(\d+),expires=(\d+),avg_ttl=(\d+)/

/**
 * 解析redis信息中的Keyspace信息
 * @param info redis info
 */
export function parseKeyspaces(info: Record<string, string>):Keyspace[] {
  const keyspaces:Keyspace[] = []
  for (let i = 0; i < 16; i++) {
    const dbKey = `db${i}`
    const value = info[dbKey]
    if (!value) {
      keyspaces.push({
        db: i,
        len: 0,
        expires: 0,
        avg_ttl: 0,
      })
      continue
    }

    const matchs = keyspaceMatch.exec(value)
    if (matchs && matchs.length > 1) {
      keyspaces.push({
        db: i,
        len: Number.parseInt(matchs[1]),
        expires: Number.parseInt(matchs[2]),
        avg_ttl: Number.parseInt(matchs[3]),
      })
    }
  }

  return keyspaces
}
