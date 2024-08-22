export function move_element<T>(arr: T[], oldIndex: number, newIndex: number): T[] {
  if (oldIndex === newIndex) {
    return arr
  }

  const newArr = [...arr]
  const element = newArr[oldIndex]

  if (newIndex > oldIndex) {
    for (let i = oldIndex; i < newIndex; i++) {
      newArr[i] = newArr[i + 1]
    }
  } else {
    for (let i = oldIndex; i > newIndex; i--) {
      newArr[i] = newArr[i - 1]
    }
  }
  newArr[newIndex] = element

  return newArr
}

export function rank_to_emoji(rank: number) {
  switch (rank) {
    case 1:
      return '🥇'
    case 2:
      return '🥈'
    case 3:
      return '🥉'
    default:
      return rank.toString()
  }
}

const emojis: string[] = [
  '😀',
  '🍉',
  '🚀',
  '🌵',
  '🐘',
  '🏠',
  '🎈',
  '⚽',
  '🦩',
  '📚',
  '🍕',
  '🚗',
  '⌚',
  '👻',
  '🌈',
  '🐠',
  '🍁',
  '🥁',
  '🔧',
  '💼',
  '🦄',
  '🍍',
  '🛴',
  '🌋',
  '🐪',
  '🏰',
  '🔭',
  '🥌',
  '🦜',
  '📷',
  '🍣',
  '🚢',
  '🗝️',
  '🧟',
  '🌠',
  '🐙',
  '🌴',
  '🎻',
  '💉',
  '🧳',
  '🦖',
  '🎡',
  '🏹',
  '👽',
  '🌪️',
  '🦔',
  '🥨',
  '🎮',
  '🧩',
  '🌁',
  '🦀',
  '🍦',
  '🚲',
  '🌊',
  '🐋',
  '🎆',
  '🎎',
  '🎹',
  '🏺',
  '🧵',
  '🦋',
  '🥑',
  '🛵',
  '🏞️',
  '🦘',
  '🌌',
  '🎨',
  '🥋',
  '🔖',
  '🥾',
  '🦂',
  '🥥',
  '🎠',
  '🏜️',
  '🦢',
  '🔋',
  '📻',
  '🛹',
  '🪂',
  '🔮',
  '🦩',
  '🍇',
  '🚂',
  '🏯',
  '🐍',
  '🪐',
  '🖌️',
  '🥊',
  '📔',
  '🪁',
  '🦚',
  '🍯',
  '🚁',
  '🗼',
  '🦥',
  '🪀',
  '🧩',
  '🎭',
  '📅',
  '🧲'
]

export function get_emoji(index: number): string {
  return emojis[index % emojis.length]
}

export interface VoteOption {
  name: string
  index: number
}

export function options_to_vote_options(options: string[]): VoteOption[] {
  return options
    .map((option, index) => {
      return {
        name: get_emoji(index) + ' ' + option,
        index: index
      }
    })
    .map((value) => ({ value, sort: Math.random() }))
    .sort((a, b) => a.sort - b.sort)
    .map(({ value }) => value)
}
