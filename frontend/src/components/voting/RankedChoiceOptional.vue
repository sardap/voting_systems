<script setup lang="ts">
import { onMounted, ref, watch, type PropType } from 'vue'

interface VoteOption {
  name: string
  index: number
}

const props = defineProps({
  options: {
    type: Object as PropType<VoteOption[]>,
    required: true
  },
  rank_title: {
    type: String,
    required: false,
    default: 'Rank'
  }
})

const emits = defineEmits<{
  (e: 'updated', value: number[]): void
}>()

const ranking = ref<string[]>(props.options.map((_) => ''))
const error = ref<string | null>(null)

watch(error, (value) => {
  if (value !== null) {
    emits('updated', [])
  }
})

function ranking_to_ballot(ranking: string[]): number[] {
  const ranked_choice = []
  for (let i = 0; i < ranking.length; i++) {
    if (ranking[i] !== '') {
      ranked_choice.push({
        rank: Number(ranking[i]),
        index: i
      })
    }
  }

  ranked_choice.sort((a, b) => a.rank - b.rank)

  return ranked_choice.map((i) => i.index)
}

watch(
  ranking,
  () => {
    console.log(ranking.value)
    // Check for non null duplicates in ranking
    // Any value that can be turned into a number is valid
    // Check for invalid values
    const invalid = ranking.value.filter((rank) => {
      return rank.length > 0 && isNaN(Number(rank))
    })
    if (invalid.length > 0) {
      error.value = `Invalid ${props.rank_title} value.`
      return
    }

    // Check for duplicates
    const seen = new Set()
    for (let i = 0; i < ranking.value.length; i++) {
      if (ranking.value[i] !== '' && seen.has(ranking.value[i])) {
        error.value = 'Duplicate rank.'
        return
      }
      seen.add(ranking.value[i])
    }

    // Make sure all ranks start at 1 and end at the number of options
    const ranks = ranking.value.filter((rank) => rank !== '').map((rank) => Number(rank))
    for (let i = 1; i <= ranks.length; i++) {
      if (!ranks.includes(i)) {
        error.value = `Missing rank ${i}.`
        return
      }
    }

    const ballot = ranking_to_ballot(ranking.value)

    error.value = null
    emits('updated', ballot)
  },
  { deep: true }
)

onMounted(() => {
  emits('updated', [])
})
</script>

<template>
  <div>
    <p class="error" v-if="error">{{ error }}</p>
    <div ref="el" class="table">
      <div class="row header">
        <div class="col rank">
          <p>{{ rank_title }}</p>
        </div>
        <div class="col">
          <p>Name</p>
        </div>
      </div>
      <div v-for="(option, i) in options" :key="i" class="row item">
        <div class="col rank">
          <input class="input-number" v-model="ranking[i]" />
        </div>
        <div class="col">
          <p>{{ option.name }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
div.row:hover {
  background-color: #e8ab8c;
}

div.row:nth-child(2n):hover {
  background-color: #e8ab8c;
}

div.row:nth-child(2n) {
  background-color: #fad8c0;
}

div.row {
  background-color: #fde5d0;
}

div.row.header {
  background-color: #a2f4b9;
}

div.table {
  display: table;
  max-width: 800px;
  width: 90%;
  border-collapse: separate;
  border-spacing: 0;
}

div.row {
  display: table-row;
}

div.col {
  display: table-cell;
  border: 1px solid black;
  padding: 8px;
  text-align: left;
  vertical-align: top;
}

div.col.rank {
  width: 10px;
}

.submit {
  margin-top: 20px;
  max-width: 800px;
  width: 70%;
  height: 80px;
}

.input-number {
  width: 3em;
  text-align: center;
}

td {
  text-align: left;
}

/* Chrome, Safari, Edge, Opera */
input[type='number']::-webkit-inner-spin-button,
input[type='number']::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

/* Firefox */
input[type='number'] {
  -moz-appearance: textfield;
}
</style>
